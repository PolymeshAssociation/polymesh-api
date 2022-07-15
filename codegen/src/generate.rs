use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

use heck::ToSnakeCase;

use indexmap::IndexMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use codec::{Decode, Encode};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};

fn segments_ident(segments: &[String]) -> TokenStream {
  let idents: Vec<_> = segments.into_iter().map(|s| format_ident!("{s}")).collect();
  quote! {
    #(#idents)::*
  }
}

struct ModuleCode {
  name: String,
  sub_modules: HashMap<String, ModuleCode>,
  types: HashMap<String, TokenStream>,
}

impl ModuleCode {
  fn new(name: String) -> Self {
    Self {
      name,
      sub_modules: HashMap::new(),
      types: HashMap::new(),
    }
  }

  fn add_type(&mut self, segments: &[String], ident: String, code: TokenStream) {
    if let Some((mod_name, segments)) = segments.split_first() {
      let entry = self.sub_modules.entry(mod_name.into());
      let sub = entry.or_insert_with(|| ModuleCode::new(mod_name.into()));
      sub.add_type(segments, ident, code);
    } else if self.name.len() > 0 {
      self.types.insert(ident, code);
    }
  }

  fn gen(mut self) -> TokenStream {
    let mut code = TokenStream::new();
    for (name, sub) in self.sub_modules.drain() {
      let ident = format_ident!("{name}");
      let sub_code = sub.gen();
      code.append_all(quote! {
        pub mod #ident {
          use super::*;
          #sub_code
        }
      });
    }
    for (_, ty_code) in self.types.drain() {
      code.append_all(ty_code);
    }
    code
  }
}

#[cfg(feature = "v14")]
mod v14 {
  use super::*;
  use frame_metadata::v14::RuntimeMetadataV14;
  use scale_info::{form::PortableForm, Field, Type, TypeDef, Variant};

  #[derive(Default)]
  struct TypeParameters {
    names: IndexMap<u32, String>,
    used: HashSet<String>,
  }

  impl TypeParameters {
    fn new(ty: &Type<PortableForm>) -> Self {
      let mut names = IndexMap::new();

      let ty_params = ty.type_params();
      if ty_params.len() > 0 {
        for p in ty_params {
          if let Some(p_ty) = p.ty() {
            let name = p.name();
            names.insert(p_ty.id(), name.into());
          }
        }
      }

      Self {
        names,
        used: HashSet::default(),
      }
    }

    fn get_param(&mut self, id: u32) -> Option<TokenStream> {
      self.names.get(&id).map(|name| {
        self.used.insert(name.to_string());
        let name = format_ident!("{name}");
        quote! { #name }
      })
    }

    fn get_type_params(&self) -> TokenStream {
      if self.names.len() > 0 {
        let params = self.names.values().map(|name| {
          let ident = format_ident!("{name}");
          quote!(#ident)
        });
        quote!(<#(#params),*>)
      } else {
        TokenStream::new()
      }
    }

    fn get_unused_params(&self) -> Option<TokenStream> {
      if self.used.len() < self.names.len() {
        let params = self
          .names
          .values()
          .filter(|name| !self.used.contains(*name))
          .map(|name| {
            let ident = format_ident!("{name}");
            quote!(#ident)
          })
          .collect::<Vec<_>>();
        // Return a tuple type with the unused params.
        if params.len() > 1 {
          Some(quote! { core::marker::PhantomData<(#(#params),*)> })
        } else {
          Some(quote! { core::marker::PhantomData<#(#params),*> })
        }
      } else {
        None
      }
    }
  }

  struct Generator {
    md: RuntimeMetadataV14,
    external_modules: HashSet<String>,
    rename_types: HashMap<String, TokenStream>,
    call: TokenStream,
  }

  impl Generator {
    fn new(md: RuntimeMetadataV14) -> Self {
      // Detect the chain runtime path.
      let runtime_ty = md.types.resolve(md.ty.id()).unwrap();
      let runtime_ns = runtime_ty.path().namespace();
      let runtime_ident = segments_ident(runtime_ns);

      let runtime_call_ty = format!("{}::Call", runtime_ns.join("::"));
      let call = quote! { #runtime_ident::Call };
      let external_modules = HashSet::from_iter(
        [
          "sp_arithmetic",
          "sp_core",
          "sp_session",
          "sp_runtime",
          "sp_version",
          "frame_support",
        ]
        .iter()
        .map(|t| t.to_string()),
      );
      let rename_types = HashMap::from_iter(
        [
          (runtime_call_ty.as_str(), quote!(WrappedCall)),
          (
            "sp_runtime::multiaddress::MultiAddress",
            quote!(::sub_api::basic_types::sp_runtime::MultiAddress),
          ),
          (
            "sp_runtime::generic::era::Era",
            quote!(::sub_api::basic_types::sp_runtime::generic::Era),
          ),
          (
            "sp_runtime::generic::header::Header",
            quote!(::sub_api::basic_types::sp_runtime::generic::Header),
          ),
          ("BTreeSet", quote!(Vec)),
          ("BTreeMap", quote!(std::collections::BTreeMap)),
          (
            "frame_support::traits::tokens::misc::BalanceStatus",
            quote!(::sub_api::basic_types::frame_support::traits::BalanceStatus),
          ),
          (
            "frame_support::storage::weak_bounded_vec::WeakBoundedVec",
            quote!(Vec),
          ),
        ]
        .into_iter()
        .map(|(name, code)| (name.to_string(), code)),
      );
      Self {
        md,
        external_modules,
        rename_types,
        call,
      }
    }

    fn is_boxed(field: &Field<PortableForm>) -> bool {
      if let Some(type_name) = field.type_name() {
        type_name.contains("Box<")
      } else {
        false
      }
    }

    fn is_compact(&self, field: &Field<PortableForm>) -> bool {
      if let Some(ty) = self.md.types.resolve(field.ty().id()) {
        match ty.type_def() {
          TypeDef::Compact(_) => true,
          _ => false,
        }
      } else {
        false
      }
    }

    fn type_name(&self, id: u32, compact_wrap: bool) -> Option<TokenStream> {
      let mut scope = TypeParameters::default();
      self.type_name_scoped(id, &mut scope, compact_wrap)
    }

    fn type_name_scoped(
      &self,
      id: u32,
      scope: &mut TypeParameters,
      compact_wrap: bool,
    ) -> Option<TokenStream> {
      if let Some(scope_type) = scope.get_param(id) {
        return Some(scope_type);
      }
      let ty = self.md.types.resolve(id)?;
      let segments = ty.path().segments();
      let full_name = segments.join("::");
      let type_ident = self
        .rename_types
        .get(&full_name)
        .cloned()
        .unwrap_or_else(|| segments_ident(segments));

      match ty.type_def() {
        TypeDef::Sequence(ty) => {
          return self
            .type_name_scoped(ty.type_param().id(), scope, true)
            .map(|elem_ty| {
              quote! { Vec<#elem_ty> }
            });
        }
        TypeDef::Array(ty) => {
          let len = ty.len() as usize;
          return self
            .type_name_scoped(ty.type_param().id(), scope, true)
            .map(|elem_ty| {
              quote! { [#elem_ty; #len] }
            });
        }
        TypeDef::Tuple(ty) => {
          let fields = ty
            .fields()
            .into_iter()
            .filter_map(|field| self.type_name_scoped(field.id(), scope, true))
            .collect::<Vec<_>>();
          return Some(quote! { (#(#fields),*) });
        }
        TypeDef::Primitive(prim) => {
          use scale_info::TypeDefPrimitive::*;
          let name = format_ident!(
            "{}",
            match prim {
              Bool => "bool",
              Char => "char",
              Str => "String",
              U8 => "u8",
              U16 => "u16",
              U32 => "u32",
              U64 => "u64",
              U128 => "u128",
              U256 => "u256",
              I8 => "i8",
              I16 => "i16",
              I32 => "i32",
              I64 => "i64",
              I128 => "i128",
              I256 => "i256",
            }
          );
          return Some(quote! { #name });
        }
        TypeDef::Compact(ty) => {
          return self
            .type_name_scoped(ty.type_param().id(), scope, true)
            .map(|ty| {
              if compact_wrap {
                quote! { ::codec::Compact<#ty> }
              } else {
                ty
              }
            });
        }
        _ => {}
      }

      let type_params = ty
        .type_params()
        .iter()
        .filter_map(|param| {
          param
            .ty()
            .map(|ty| self.type_name_scoped(ty.id(), scope, true))
        })
        .collect::<Vec<_>>();

      if type_params.len() > 0 {
        Some(quote! {
          #type_ident<#(#type_params),*>
        })
      } else {
        Some(type_ident)
      }
    }

    fn gen_func(
      &self,
      mod_name: &str,
      _mod_idx: u8,
      mod_call_ty: u32,
      md: &Variant<PortableForm>,
    ) -> TokenStream {
      let mod_call_ident = format_ident!("{mod_name}");
      let mod_call = self.type_name(mod_call_ty, true).unwrap();
      let func_name = md.name();
      let func_ident = format_ident!("{}", func_name.to_snake_case());

      let mut fields = TokenStream::new();
      let mut field_names = TokenStream::new();
      for (idx, field) in md.fields().iter().enumerate() {
        let name = field
          .name()
          .map(|n| format_ident!("{n}"))
          .unwrap_or_else(|| format_ident!("param_{idx}"));
        let type_name = self
          .type_name(field.ty().id(), false)
          .expect("Missing Extrinsic param type");
        fields.append_all(quote! {#name: #type_name,});
        if Self::is_boxed(field) {
          field_names.append_all(quote! {#name: ::std::boxed::Box::new(#name),});
        } else {
          field_names.append_all(quote! {#name,});
        }
      }

      let call_ty = &self.call;
      if md.fields().len() > 0 {
        quote! {
          pub fn #func_ident(&self, #fields) -> ::sub_api::error::Result<super::super::WrappedCall> {
            Ok(types::#call_ty::#mod_call_ident(types::#mod_call::#func_ident { #field_names }).into())
          }
        }
      } else {
        quote! {
          pub fn #func_ident(&self, #fields) -> ::sub_api::error::Result<super::super::WrappedCall> {
            Ok(types::#call_ty::#mod_call_ident(types::#mod_call::#func_ident).into())
          }
        }
      }
    }

    fn gen_module(
      &self,
      md: &frame_metadata::v14::PalletMetadata<PortableForm>,
    ) -> (Ident, TokenStream) {
      let mod_idx = md.index;
      let mod_name = &md.name;
      let mod_ident = format_ident!("{}", mod_name.to_snake_case());

      let mut call_fields = TokenStream::new();
      //let mut query_fields = TokenStream::new();

      // Generate module functions.
      if let Some(calls) = &md.calls {
        let call_ty = self
          .md
          .types
          .resolve(calls.ty.id())
          .expect("Missing Pallet call type");
        match call_ty.type_def() {
          TypeDef::Variant(v) => {
            let mod_call_ty = calls.ty.id();
            for v in v.variants() {
              let code = self.gen_func(mod_name, mod_idx, mod_call_ty, v);
              call_fields.append_all(code);
            }
          }
          _ => {
            unimplemented!("Only Variant type supported for Pallet Call type.");
          }
        }
      }

      let code = quote! {
        pub mod #mod_ident {
          use super::*;

          #[derive(Clone, Debug)]
          pub struct CallApi<'a> {
            api: &'a super::super::Api,
          }

          impl<'a> CallApi<'a> {
            #call_fields
          }

          impl<'a> From<&'a super::super::Api> for CallApi<'a> {
              fn from(api: &'a super::super::Api) -> Self {
                  Self { api }
              }
          }

          /*
          #[derive(Clone, Debug, Default)]
          pub struct QueryApi;

          impl QueryApi {
            #query_fields
          }
          */
        }
      };
      (mod_ident, code)
    }

    fn gen_struct_fields(
      &self,
      fields: &[Field<PortableForm>],
      scope: &mut TypeParameters,
    ) -> Option<(bool, TokenStream)> {
      let mut is_tuple = false;
      let mut named = Vec::new();
      let mut unnamed = Vec::new();

      // Check for unit type (i.e. empty field list).
      if fields.len() == 0 {
        return Some((true, quote! {}));
      }

      for field in fields {
        let mut field_ty = self.type_name_scoped(field.ty().id(), scope, false)?;
        if Self::is_boxed(field) {
          field_ty = quote!(::std::boxed::Box<#field_ty>);
        }
        let attr = if self.is_compact(field) {
          quote! { #[codec(compact)]}
        } else {
          quote! {}
        };
        unnamed.push(quote! { #attr pub #field_ty });
        if let Some(name) = field.name() {
          let name = format_ident!("{name}");
          named.push(quote! { #attr pub #name: #field_ty });
        } else {
          // If there are any unnamed fields, then make it a tuple.
          is_tuple = true;
        }
      }

      if is_tuple {
        Some((true, quote! { #(#unnamed),* }))
      } else {
        Some((
          false,
          quote! {
            #(#named),*
          },
        ))
      }
    }

    fn gen_enum_fields(
      &self,
      fields: &[Field<PortableForm>],
      scope: &mut TypeParameters,
    ) -> Option<TokenStream> {
      let mut is_tuple = false;
      let mut named = Vec::new();
      let mut unnamed = Vec::new();

      // Check for unit type (i.e. empty field list).
      if fields.len() == 0 {
        return Some(quote! {,});
      }

      for field in fields {
        let mut field_ty = self.type_name_scoped(field.ty().id(), scope, false)?;
        if Self::is_boxed(field) {
          field_ty = quote!(::std::boxed::Box<#field_ty>);
        }
        let attr = if self.is_compact(field) {
          quote! { #[codec(compact)]}
        } else {
          quote! {}
        };
        unnamed.push(quote! { #attr #field_ty });
        if let Some(name) = field.name() {
          let name = format_ident!("{name}");
          named.push(quote! { #attr #name: #field_ty });
        } else {
          // If there are any unnamed fields, then make it a tuple.
          is_tuple = true;
        }
      }

      if is_tuple {
        Some(quote! { (#(#unnamed),*), })
      } else {
        Some(quote! {
          {
            #(#named),*
          },
        })
      }
    }

    fn gen_type(&self, ty: &Type<PortableForm>) -> Option<(String, TokenStream)> {
      let ident = ty.path().ident()?;
      let ty_ident = format_ident!("{ident}");
      let mut scope = TypeParameters::new(ty);
      Some((
        ident,
        match ty.type_def() {
          TypeDef::Composite(struct_ty) => {
            let (is_tuple, mut fields) = self.gen_struct_fields(struct_ty.fields(), &mut scope)?;
            if let Some(unused_params) = scope.get_unused_params() {
              if is_tuple {
                fields.append_all(quote! {
                  , #unused_params
                });
              } else {
                fields.append_all(quote! {
                  , _phantom_data: #unused_params
                });
              }
            }
            let params = scope.get_type_params();
            if is_tuple {
              quote! {
                #[derive(Clone, Debug)]
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct #ty_ident #params (#fields);
              }
            } else {
              quote! {
                #[derive(Clone, Debug)]
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct #ty_ident #params { #fields }
              }
            }
          }
          TypeDef::Variant(enum_ty) => {
            let mut variants = TokenStream::new();
            for variant in enum_ty.variants() {
              let idx = variant.index();
              let name = format_ident!("{}", variant.name());
              let fields = self.gen_enum_fields(variant.fields(), &mut scope)?;
              variants.append_all(quote! {
                #[codec(index = #idx)]
                #name #fields
              });
            }
            if let Some(unused_params) = scope.get_unused_params() {
              variants.append_all(quote! {
                PhantomDataVariant(#unused_params)
              });
            }
            let params = scope.get_type_params();
            quote! {
              #[derive(Clone, Debug)]
              #[derive(::codec::Encode, ::codec::Decode)]
              #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
              pub enum #ty_ident #params {
                #variants
              }
            }
          }
          _ => {
            return None;
          }
        },
      ))
    }

    fn generate_types(&self) -> TokenStream {
      // Start with empty namespace.
      let mut modules = ModuleCode::new("".into());

      for ty in self.md.types.types() {
        let ty = ty.ty();
        // Only generate type code for types with namespaces.  Basic rust types like
        // `Result` and `Option` have no namespace.
        if let Some(ns_top) = ty.path().namespace().first() {
          // Don't generate code for external types.
          if !self.external_modules.contains(ns_top) {
            if let Some((ident, code)) = self.gen_type(ty) {
              modules.add_type(ty.path().namespace(), ident, code);
            }
          }
        }
      }

      modules.gen()
    }

    pub fn generate(self) -> TokenStream {
      let mut call_fields = TokenStream::new();
      //let mut query_fields = TokenStream::new();

      // Generate module code.
      let modules: Vec<_> = self
        .md
        .pallets
        .iter()
        .map(|m| {
          let (ident, code) = self.gen_module(m);
          call_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::CallApi {
              api::#ident::CallApi::from(self.api)
            }
          });
          /*
          query_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::QueryApi {
              api::#ident::QueryApi::default()
            }
          });
          */

          code
        })
        .collect();

      let types_code = self.generate_types();

      let metadata_bytes = self.md.encode();
      let call_ty = &self.call;
      quote! {
        use ::codec::Decode;

        pub const API_METADATA_BYTES: &'static [u8] = &[ #(#metadata_bytes,)* ];
        ::lazy_static::lazy_static! {
            pub static ref API_METADATA: ::frame_metadata::v14::RuntimeMetadataV14 = {
              ::frame_metadata::v14::RuntimeMetadataV14::decode(&mut &API_METADATA_BYTES[..])
                  .expect("Shouldn't be able to fail")
            };
        }

        #[derive(Clone, Debug)]
        #[derive(::codec::Encode, ::codec::Decode)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct WrappedCall {
          call: types::#call_ty,
        }

        impl From<types::#call_ty> for WrappedCall {
            fn from(call: types::#call_ty) -> Self {
                Self { call }
            }
        }

        impl From<WrappedCall> for types::#call_ty {
            fn from(wrapped: WrappedCall) -> Self {
                wrapped.call
            }
        }

        impl From<&WrappedCall> for types::#call_ty {
            fn from(wrapped: &WrappedCall) -> Self {
                wrapped.call.clone()
            }
        }

        pub mod types {
          use super::WrappedCall;
          #types_code
        }

        pub mod api {
          use super::types;
          use super::types::*;
          use super::WrappedCall;

          #( #modules )*
        }

        #[derive(Debug)]
        pub struct Api {
          metadata: ::std::sync::Arc<::std::sync::RwLock<::frame_metadata::RuntimeMetadataPrefixed>>,
        }

        impl From<::frame_metadata::RuntimeMetadataPrefixed> for Api {
            fn from(metadata: ::frame_metadata::RuntimeMetadataPrefixed) -> Self {
                Self {
                  metadata: ::std::sync::Arc::new(::std::sync::RwLock::new(metadata)),
                }
            }
        }

        impl Api {
          pub fn call(&self) -> CallApi {
            CallApi { api: self }
          }
        }

        #[derive(Clone, Debug)]
        pub struct CallApi<'a> {
          api: &'a Api,
        }

        impl<'a> CallApi<'a> {
          #call_fields
        }

        /*
        #[derive(Clone, Default)]
        pub struct QueryApi {
        }

        impl QueryApi {
          #query_fields
        }
          */
      }
    }
  }

  pub fn generate(md: RuntimeMetadataV14) -> TokenStream {
    Generator::new(md).generate()
  }
}

pub fn generate(metadata: RuntimeMetadataPrefixed) -> Result<TokenStream> {
  match metadata.1 {
    #[cfg(feature = "v14")]
    RuntimeMetadata::V14(v14) => Ok(v14::generate(v14)),
    _ => {
      return Err(anyhow!("Unsupported metadata version").into());
    }
  }
}

pub fn macro_codegen(mut buf: &[u8], mod_ident: TokenStream) -> Result<TokenStream> {
  let metadata = RuntimeMetadataPrefixed::decode(&mut buf)?;

  let code = generate(metadata)?;
  Ok(quote! {
    pub mod #mod_ident {
      #code
    }
  })
}

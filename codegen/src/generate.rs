use std::collections::HashMap;

use anyhow::{anyhow, Result};

use heck::ToSnakeCase;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};

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

  struct TypeParameters {
    names: HashMap<u32, String>,
  }

  impl TypeParameters {
    fn new(ty: &Type<PortableForm>) -> (TokenStream, Self) {
      let mut names = HashMap::new();

      let ty_params = ty.type_params();
      let code = if ty_params.len() > 0 {
        let mut params = Vec::with_capacity(ty_params.len());
        for p in ty_params {
          if let Some(p_ty) = p.ty() {
            let name = p.name();
            let ident = format_ident!("{name}");
            names.insert(p_ty.id(), name.into());
            params.push(quote! { #ident })
          }
        }
        quote! { <#(#params),*> }
      } else {
        TokenStream::new()
      };

      (code, Self { names })
    }

    fn get_param(&self, id: u32) -> Option<TokenStream> {
      self.names.get(&id).map(|name| {
        let name = format_ident!("{name}");
        quote! { #name }
      })
    }
  }

  struct Generator {
    md: RuntimeMetadataV14,
    call: TokenStream,
  }

  impl Generator {
    fn new(md: RuntimeMetadataV14, runtime: &str) -> Self {
      let runtime_ident = format_ident!("{runtime}");
      let call = quote! { #runtime_ident::runtime::Call };
      Self { md, call }
    }

    fn type_name(&self, id: u32) -> Option<TokenStream> {
      let ty = self.md.types.resolve(id)?;
      let segments: Vec<_> = ty
        .path()
        .segments()
        .iter()
        .map(|s| format_ident!("{s}"))
        .collect();

      match ty.type_def() {
        TypeDef::Sequence(ty) => {
          return self.type_name(ty.type_param().id()).map(|elem_ty| {
            quote! { Vec<#elem_ty> }
          });
        }
        TypeDef::Array(ty) => {
          let len = ty.len();
          return self.type_name(ty.type_param().id()).map(|elem_ty| {
            quote! { [#elem_ty; #len] }
          });
        }
        TypeDef::Tuple(_ty) => {
          return Some(quote! { () });
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
          return self.type_name(ty.type_param().id()).map(|ty| {
            quote! { codec::Compact<#ty> }
          });
        }
        _ => {}
      }

      let type_params = ty
        .type_params()
        .iter()
        .filter_map(|param| param.ty().map(|ty| self.type_name(ty.id())))
        .collect::<Vec<_>>();

      if type_params.len() > 0 {
        Some(quote! {
          #(#segments)::*<#(#type_params),*>
        })
      } else {
        Some(quote! {
          #(#segments)::*
        })
      }
    }

    fn type_name_scoped(&self, id: u32, scope: &TypeParameters) -> Option<TokenStream> {
      scope.get_param(id).or_else(|| self.type_name(id))
    }

    fn gen_func(
      &self,
      mod_name: &str,
      _mod_idx: u8,
      mod_call_ty: u32,
      md: &Variant<PortableForm>,
    ) -> TokenStream {
      let mod_call_ident = format_ident!("{mod_name}");
      let mod_call = self.type_name(mod_call_ty).unwrap();
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
          .type_name(field.ty().id())
          .expect("Missing Extrinsic param type");
        fields.append_all(quote! {#name: #type_name,});
        field_names.append_all(quote! {#name,});
      }

      let call_ty = &self.call;
      if md.fields().len() > 0 {
        quote! {
          pub fn #func_ident(&self, #fields) -> #call_ty {
            #call_ty::#mod_call_ident(#mod_call::#func_ident { #field_names })
          }
        }
      } else {
        quote! {
          pub fn #func_ident(&self, #fields) -> #call_ty {
            #call_ty::#mod_call_ident(#mod_call::#func_ident)
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
        mod #mod_ident {
          #[derive(Clone, Default)]
          pub struct CallApi;

          impl CallApi {
            #call_fields
          }

          /*
          #[derive(Clone, Default)]
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
      type_params: &TypeParameters,
    ) -> Option<TokenStream> {
      let mut is_tuple = false;
      let mut named = Vec::new();
      let mut unnamed = Vec::new();

      // Check for unit type (i.e. empty field list).
      if fields.len() == 0 {
        return Some(quote! {;});
      }

      for field in fields {
        let field_ty = self.type_name_scoped(field.ty().id(), &type_params)?;
        unnamed.push(quote! { pub #field_ty });
        if let Some(name) = field.name() {
          let name = format_ident!("{name}");
          named.push(quote! { pub #name: #field_ty });
        } else {
          // If there are any unnamed fields, then make it a tuple.
          is_tuple = true;
        }
      }

      if is_tuple {
        Some(quote! { (#(#unnamed),*); })
      } else {
        Some(quote! {
          {
            #(#named),*
          }
        })
      }
    }

    fn gen_enum_fields(
      &self,
      fields: &[Field<PortableForm>],
      type_params: &TypeParameters,
    ) -> Option<TokenStream> {
      let mut is_tuple = false;
      let mut named = Vec::new();
      let mut unnamed = Vec::new();

      // Check for unit type (i.e. empty field list).
      if fields.len() == 0 {
        return Some(quote! {,});
      }

      for field in fields {
        let field_ty = self.type_name_scoped(field.ty().id(), &type_params)?;
        unnamed.push(quote! { #field_ty });
        if let Some(name) = field.name() {
          let name = format_ident!("{name}");
          named.push(quote! { #name: #field_ty });
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
      let (params, type_params) = TypeParameters::new(ty);
      Some((
        ident,
        match ty.type_def() {
          TypeDef::Composite(struct_ty) => {
            let fields = self.gen_struct_fields(struct_ty.fields(), &type_params)?;
            quote! {
              #[derive(codec::Encode, codec::Decode)]
              #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
              pub struct #ty_ident #params #fields
            }
          }
          TypeDef::Variant(enum_ty) => {
            let mut variants = TokenStream::new();
            for variant in enum_ty.variants() {
              let idx = variant.index();
              let name = format_ident!("{}", variant.name());
              let fields = self.gen_enum_fields(variant.fields(), &type_params)?;
              variants.append_all(quote! {
                #[codec(index = #idx)]
                #name #fields
              });
            }
            quote! {
              #[derive(codec::Encode, codec::Decode)]
              #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        if let Some((ident, code)) = self.gen_type(ty) {
          modules.add_type(ty.path().namespace(), ident, code);
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
            pub #ident: api::#ident::CallApi,
          });
          /*
          query_fields.append_all(quote! {
            pub #ident: api::#ident::QueryApi,
          });
          */

          code
        })
        .collect();

      let types_code = self.generate_types();

      quote! {
        pub mod types {
          #types_code
        }

        pub mod api {
          use super::types::*;
          #( #modules )*
        }

          /*
        #[derive(Clone, Default)]
        pub struct Api {
          call: CallApi,
          query: QueryApi,
        }

        impl Api {
          pub fn new() -> Self {
            Self::default()
          }
        }

        #[derive(Clone, Default)]
        pub struct CallApi {
          #call_fields
        }

        #[derive(Clone, Default)]
        pub struct QueryApi {
          #query_fields
        }
          */
      }
    }
  }

  pub fn generate(md: RuntimeMetadataV14) -> TokenStream {
    Generator::new(md, "polymesh_runtime_develop").generate()
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

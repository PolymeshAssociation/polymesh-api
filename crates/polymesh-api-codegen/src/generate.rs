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
  use frame_metadata::v14::{
    RuntimeMetadataV14, StorageEntryMetadata, StorageEntryModifier, StorageEntryType, StorageHasher,
  };
  use scale_info::{form::PortableForm, Field, Path, Type, TypeDef, Variant};

  #[derive(Default)]
  struct TypeParameters {
    names: IndexMap<u32, String>,
    used: HashSet<String>,
    need_bounds: HashMap<u32, HashMap<String, TokenStream>>,
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
        used: Default::default(),
        need_bounds: Default::default(),
      }
    }

    fn add_param_bounds(&mut self, id: u32, bound_name: &str, type_bound: TokenStream) -> bool {
      if self.names.contains_key(&id) {
        let bounds = self.need_bounds.entry(id).or_default();
        bounds.insert(bound_name.to_string(), type_bound);
        true
      } else {
        false
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
        let params = self.names.iter().map(|(id, name)| {
          let ident = format_ident!("{name}");
          if let Some(with_bounds) = self.need_bounds.get(&id) {
            let bounds: Vec<_> = with_bounds.values().collect();
            quote!(#ident: #(#bounds) + *)
          } else {
            quote!(#ident)
          }
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
    ord_types: HashSet<String>,
    runtime_namespace: Vec<String>,
    call: TokenStream,
    event: TokenStream,
  }

  impl Generator {
    fn new(md: RuntimeMetadataV14) -> Self {
      // Detect the chain runtime path.
      let runtime_ty = md.types.resolve(md.ty.id()).unwrap();
      let runtime_namespace = runtime_ty.path().namespace();
      let runtime_ident = segments_ident(runtime_namespace);

      let call = quote! { #runtime_ident::Call };
      let event = quote! { #runtime_ident::Event };
      let external_modules = HashSet::from_iter(["sp_version"].iter().map(|t| t.to_string()));
      let rename_types = HashMap::from_iter(
        [
          (
            "sp_core::crypto::AccountId32",
            quote!(::polymesh_api_client::AccountId),
          ),
          (
            "sp_runtime::multiaddress::MultiAddress",
            quote!(::polymesh_api_client::MultiAddress),
          ),
          (
            "sp_runtime::generic::era::Era",
            quote!(::polymesh_api_client::Era),
          ),
          (
            "sp_arithmetic::per_things::Perbill",
            quote!(::polymesh_api_client::per_things::Perbill),
          ),
          (
            "sp_arithmetic::per_things::Permill",
            quote!(::polymesh_api_client::per_things::Permill),
          ),
          (
            "sp_arithmetic::per_things::PerU16",
            quote!(::polymesh_api_client::per_things::PerU16),
          ),
          (
            "sp_arithmetic::per_things::Percent",
            quote!(::polymesh_api_client::per_things::Percent),
          ),
          ("BTreeSet", quote!(std::collections::BTreeSet)),
          ("BTreeMap", quote!(std::collections::BTreeMap)),
          (
            "frame_support::storage::weak_bounded_vec::WeakBoundedVec",
            quote!(Vec),
          ),
          (
            "frame_support::storage::bounded_vec::BoundedVec",
            quote!(Vec),
          ),
          (
            "frame_system::EventRecord",
            quote!(::polymesh_api_client::EventRecord),
          ),
        ]
        .into_iter()
        .map(|(name, code)| (name.to_string(), code)),
      );

      let mut gen = Self {
        runtime_namespace: runtime_namespace.iter().cloned().collect(),
        md,
        external_modules,
        rename_types,
        ord_types: Default::default(),
        call,
        event,
      };
      // Try a limited number of types to mark all types needing the `Ord` type.
      let mut ord_type_ids = HashSet::new();
      for _ in 0..10 {
        if !gen.check_for_ord_types(&mut ord_type_ids) {
          // Finished, no new ord types.
          break;
        }
      }
      // Convert ord type ids to full names.
      for id in ord_type_ids {
        match gen.id_to_full_name(id) {
          Some(name) if name != "Option" => {
            gen.ord_types.insert(name);
          }
          _ => (),
        }
      }

      gen
    }

    fn id_to_full_name(&self, id: u32) -> Option<String> {
      let ty = self.md.types.resolve(id)?;
      let segments = ty.path().segments();
      if segments.len() > 0 {
        Some(segments.join("::"))
      } else {
        None
      }
    }

    fn check_for_ord_types(&self, ord_type_ids: &mut HashSet<u32>) -> bool {
      let count = ord_type_ids.len();
      for ty in self.md.types.types() {
        let id = ty.id();
        let ty = ty.ty();
        let full_name = self.id_to_full_name(id).unwrap_or_default();
        // Check for `BTreeSet` and `BTreeMap`.
        match full_name.as_str() {
          "BTreeSet" | "BTreeMap" => {
            // Mark the first type parameter as needing `Ord`.
            ty.type_params()
              .first()
              .and_then(|param| param.ty())
              .map(|ty| {
                ord_type_ids.insert(ty.id());
              });
            continue;
          }
          _ => (),
        }
        // Check if this type needs `Ord`.
        if ord_type_ids.contains(&id) {
          // Mark fields and used types as needing `Ord`.
          match ty.type_def() {
            TypeDef::Composite(struct_ty) => {
              for field in struct_ty.fields() {
                ord_type_ids.insert(field.ty().id());
              }
            }
            TypeDef::Variant(enum_ty) => {
              for variant in enum_ty.variants() {
                for field in variant.fields() {
                  ord_type_ids.insert(field.ty().id());
                }
              }
            }
            TypeDef::Sequence(ty) => {
              ord_type_ids.insert(ty.type_param().id());
            }
            TypeDef::Array(ty) => {
              ord_type_ids.insert(ty.type_param().id());
            }
            TypeDef::Tuple(ty) => {
              for field in ty.fields() {
                ord_type_ids.insert(field.id());
              }
            }
            TypeDef::Primitive(_) => (),
            TypeDef::Compact(ty) => {
              ord_type_ids.insert(ty.type_param().id());
            }
            _ => {}
          }
        }
      }
      let new_count = ord_type_ids.len();
      count != new_count
    }

    fn is_boxed(field: &Field<PortableForm>) -> bool {
      if let Some(type_name) = field.type_name() {
        type_name.contains("Box<")
      } else {
        false
      }
    }

    fn need_field_attributes(&self, field: &Field<PortableForm>) -> TokenStream {
      if let Some(ty) = self.md.types.resolve(field.ty().id()) {
        match ty.type_def() {
          TypeDef::Compact(_) => {
            return quote! { #[codec(compact)] };
          }
          TypeDef::Array(ty) => {
            let len = ty.len() as usize;
            if len > 32 {
              return quote! { #[cfg_attr(feature = "serde", serde(with = "::serde_big_array::BigArray"))] };
            }
          }
          _ => (),
        }
      }
      quote! {}
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
      let is_btree = match full_name.as_str() {
        "BTreeSet" | "BTreeMap" => true,
        _ => false,
      };
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

      // Check if `BTreeSet` or `BTreeMap` use a scoped paramter for the key.
      if is_btree {
        ty.type_params()
          .first()
          .and_then(|param| param.ty())
          .map(|ty| scope.add_param_bounds(ty.id(), "Ord", quote!(Ord)));
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

    fn gen_storage_func(
      &self,
      mod_prefix: &str,
      md: &StorageEntryMetadata<PortableForm>,
    ) -> TokenStream {
      let storage_name = &md.name;
      let storage_ident = format_ident!("{}", storage_name.to_snake_case());
      let mut key_prefix = Vec::with_capacity(512);
      key_prefix.extend(sp_core_hashing::twox_128(mod_prefix.as_bytes()));
      key_prefix.extend(sp_core_hashing::twox_128(storage_name.as_bytes()));

      let (hashers, value_ty) = match &md.ty {
        StorageEntryType::Plain(value) => (vec![], value.id()),
        StorageEntryType::Map {
          hashers,
          key,
          value,
        } => match hashers.as_slice() {
          [hasher] => {
            // 1 key.
            (vec![(key, hasher)], value.id())
          }
          hashers => {
            // >=2 keys.
            let keys_ty = self.md.types.resolve(key.id()).unwrap();
            let key_hashers = if let TypeDef::Tuple(ty) = keys_ty.type_def() {
              ty.fields().iter().zip(hashers).collect()
            } else {
              vec![]
            };
            (key_hashers, value.id())
          }
        },
      };
      let keys_len = hashers.len();
      let mut keys = TokenStream::new();
      let mut hashing = TokenStream::new();
      for (idx, (key, hasher)) in hashers.into_iter().enumerate() {
        let key_ident = format_ident!("key_{}", idx);
        let type_name = self
          .type_name(key.id(), false)
          .expect("Missing Storage key type");
        keys.append_all(quote! {#key_ident: #type_name,});
        hashing.append_all(match hasher {
          StorageHasher::Blake2_128 => quote! {
            buf.extend(::polymesh_api_client::sp_core::hashing::blake2_128(&#key_ident.encode()));
          },
          StorageHasher::Blake2_256 => quote! {
            buf.extend(::polymesh_api_client::sp_core::hashing::blake2_256(&#key_ident.encode()));
          },
          StorageHasher::Blake2_128Concat => quote! {
            let key = #key_ident.encode();
            buf.extend(::polymesh_api_client::sp_core::hashing::blake2_128(&key));
            buf.extend(key.into_iter());
          },
          StorageHasher::Twox128 => quote! {
            buf.extend(::polymesh_api_client::sp_core::hashing::twox_128(&#key_ident.encode()));
          },
          StorageHasher::Twox256 => quote! {
            buf.extend(::polymesh_api_client::sp_core::hashing::twox_256(&#key_ident.encode()));
          },
          StorageHasher::Twox64Concat => quote! {
            let key = #key_ident.encode();
            buf.extend(::polymesh_api_client::sp_core::hashing::twox_64(&key));
            buf.extend(key.into_iter());
          },
          StorageHasher::Identity => quote! {
            buf.extend(#key_ident.encode());
          },
        });
      }
      let value_ty = if mod_prefix == "System" && storage_name == "Events" {
        let event_ty = &self.event;
        quote!(Vec<::polymesh_api_client::EventRecord<#event_ty>>)
      } else {
        self.type_name(value_ty, false).unwrap()
      };

      let (return_ty, return_value) = match md.modifier {
        StorageEntryModifier::Optional => (quote! { Option<#value_ty>}, quote! { Ok(value) }),
        StorageEntryModifier::Default => {
          let default_value = &md.default;
          (
            quote! { #value_ty },
            quote! {
              Ok(value.unwrap_or_else(|| {
                use ::codec::Decode;
                const DEFAULT: &'static [u8] = &[#(#default_value,)*];
                <#value_ty>::decode(&mut &DEFAULT[..]).unwrap()
              }))
            },
          )
        }
      };

      let docs = &md.docs;
      if keys_len > 0 {
        quote! {
          #(#[doc = #docs])*
          pub async fn #storage_ident(&self, #keys) -> ::polymesh_api_client::error::Result<#return_ty> {
            use ::codec::Encode;
            let mut buf = Vec::with_capacity(512);
            buf.extend([#(#key_prefix,)*]);
            #hashing
            let key = ::polymesh_api_client::StorageKey(buf);
            let value = self.api.client.get_storage_by_key(key, self.at).await?;
            #return_value
          }
        }
      } else {
        quote! {
          #(#[doc = #docs])*
          pub async fn #storage_ident(&self) -> ::polymesh_api_client::error::Result<#return_ty> {
            let key = ::polymesh_api_client::StorageKey(vec![#(#key_prefix,)*]);
            let value = self.api.client.get_storage_by_key(key, self.at).await?;
            #return_value
          }
        }
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
      let mod_call = self.type_name(mod_call_ty, false).unwrap();
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

      let docs = md.docs();
      let call_ty = &self.call;
      if md.fields().len() > 0 {
        quote! {
          #(#[doc = #docs])*
          pub fn #func_ident(&self, #fields) -> ::polymesh_api_client::error::Result<super::super::WrappedCall<'api>> {
            self.api.wrap_call(types::#call_ty::#mod_call_ident(types::#mod_call::#func_ident { #field_names }))
          }
        }
      } else {
        quote! {
          #(#[doc = #docs])*
          pub fn #func_ident(&self) -> ::polymesh_api_client::error::Result<super::super::WrappedCall<'api>> {
            self.api.wrap_call(types::#call_ty::#mod_call_ident(types::#mod_call::#func_ident))
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
      let mut query_fields = TokenStream::new();

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

      // Generate module storage query functions.
      if let Some(storage) = &md.storage {
        let mod_prefix = &storage.prefix;
        for md in &storage.entries {
          let code = self.gen_storage_func(mod_prefix, md);
          query_fields.append_all(code);
        }
      }

      let code = quote! {
        pub mod #mod_ident {
          use super::*;

          #[derive(Clone, Debug)]
          pub struct CallApi<'api> {
            api: &'api super::super::Api,
          }

          impl<'api> CallApi<'api> {
            #call_fields
          }

          impl<'api> From<&'api super::super::Api> for CallApi<'api> {
            fn from(api: &'api super::super::Api) -> Self {
              Self { api }
            }
          }

          #[derive(Clone, Debug)]
          pub struct QueryApi<'api> {
            pub(crate) api: &'api super::super::Api,
            pub(crate) at: Option<::polymesh_api_client::BlockHash>,
          }

          impl<'api> QueryApi<'api> {
            #query_fields
          }
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
        let attr = self.need_field_attributes(field);
        unnamed.push(quote! { #attr pub #field_ty });
        if let Some(name) = field.name() {
          let docs = field.docs();
          let name = format_ident!("{name}");
          named.push(quote! {
              #(#[doc = #docs])*
              #attr
              pub #name: #field_ty
          });
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

    fn gen_enum_match_fields(&self, fields: &[Field<PortableForm>]) -> TokenStream {
      let mut is_tuple = false;
      let mut unnamed = Vec::new();

      // Check for unit type (i.e. empty field list).
      if fields.len() == 0 {
        return quote!();
      }

      for field in fields {
        unnamed.push(quote!(_));
        if field.name().is_none() {
          // If there are any unnamed fields, then make it a tuple.
          is_tuple = true;
        }
      }

      if is_tuple {
        quote! { (#(#unnamed),*) }
      } else {
        quote! {
          {
            ..
          }
        }
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
        let docs = field.docs();
        let attr = self.need_field_attributes(field);
        unnamed.push(quote! {
            #(#[doc = #docs])*
            #attr
            #field_ty
        });
        if let Some(name) = field.name() {
          let name = format_ident!("{name}");
          named.push(quote! {
              #(#[doc = #docs])*
              #attr
              #name: #field_ty
          });
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

    fn gen_enum_match_arms_fn<F: Fn(&str, Option<&Variant<PortableForm>>) -> TokenStream>(
      &self,
      mod_name: &str,
      fields: &[Field<PortableForm>],
      f: F,
    ) -> Option<TokenStream> {
      let mod_ident = format_ident!("{}", mod_name);
      let mut code = TokenStream::new();

      // Only support pallet enum types with a single field.
      if fields.len() != 1 {
        let arm_code = f(mod_name, None);
        return Some(quote! {
          #mod_ident(_) => {
            #arm_code
          },
        });
      }
      let field = &fields[0];

      let field_ty = self.md.types.resolve(field.ty().id()).unwrap();
      let field_ty_ident = segments_ident(field_ty.path().segments());
      match field_ty.type_def() {
        TypeDef::Variant(enum_ty) => {
          let variants = enum_ty.variants();
          if variants.len() == 0 {
            let arm_code = f(mod_name, None);
            code.append_all(quote! {
              #mod_ident(_) => {
                #arm_code
              },
            });
          }
          for variant in variants {
            let v_name = variant.name();
            let v_ident = format_ident!("{}", v_name);
            let match_fields = self.gen_enum_match_fields(variant.fields());
            let arm_code = f(mod_name, Some(variant));
            code.append_all(quote! {
              #mod_ident(#field_ty_ident :: #v_ident #match_fields) => {
                #arm_code
              },
            });
          }
        }
        _ => {
          unimplemented!("Only enum types are supported");
        }
      }

      Some(code)
    }

    fn is_runtime_type(&self, path: &Path<PortableForm>) -> Option<&'static str> {
      if self.runtime_namespace == path.namespace() {
        let ident = path.ident();
        match ident.as_deref() {
          Some("Event") => Some("Event"),
          Some("Call") => Some("Call"),
          _ => None,
        }
      } else {
        None
      }
    }

    fn gen_type(&self, id: u32, ty: &Type<PortableForm>) -> Option<(String, TokenStream)> {
      let path = ty.path();
      let ident = path.ident()?;
      let ty_ident = format_ident!("{ident}");
      let mut scope = TypeParameters::new(ty);
      let full_name = self.id_to_full_name(id)?;
      let derive_ord = if self.ord_types.contains(&full_name) {
        quote! {
          #[derive(PartialOrd, Ord)]
        }
      } else {
        quote!()
      };
      let docs = ty.docs();
      let (mut code, params) = match ty.type_def() {
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
            (
              quote! {
                #(#[doc = #docs])*
                #[derive(Clone, Debug, PartialEq, Eq)]
                #derive_ord
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct #ty_ident #params (#fields);
              },
              params,
            )
          } else {
            (
              quote! {
                #(#[doc = #docs])*
                #[derive(Clone, Debug, PartialEq, Eq)]
                #derive_ord
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
                pub struct #ty_ident #params { #fields }
              },
              params,
            )
          }
        }
        TypeDef::Variant(enum_ty) => {
          let mut variants = TokenStream::new();
          for variant in enum_ty.variants() {
            let idx = variant.index();
            let docs = variant.docs();
            let name = variant.name();
            let ident = format_ident!("{}", name);
            let fields = self.gen_enum_fields(variant.fields(), &mut scope)?;
            variants.append_all(quote! {
              #(#[doc = #docs])*
              #[codec(index = #idx)]
              #ident #fields
            });
          }
          if let Some(unused_params) = scope.get_unused_params() {
            variants.append_all(quote! {
              PhantomDataVariant(#unused_params)
            });
          }
          let params = scope.get_type_params();
          (
            quote! {
              #(#[doc = #docs])*
              #[derive(Clone, Debug, PartialEq, Eq)]
              #derive_ord
              #[derive(::codec::Encode, ::codec::Decode)]
              #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
              pub enum #ty_ident #params {
                #variants
              }
            },
            params,
          )
        }
        _ => {
          return None;
        }
      };
      match (self.is_runtime_type(path), ty.type_def()) {
        (Some("Event") | Some("Call"), TypeDef::Variant(enum_ty)) => {
          let mut as_str_arms = TokenStream::new();
          for variant in enum_ty.variants() {
            let mod_name = variant.name();
            as_str_arms.append_all(self.gen_enum_match_arms_fn(
              mod_name,
              variant.fields(),
              |mod_name, variant| {
                let name = if let Some(variant) = variant {
                  format!("{}.{}", mod_name, variant.name())
                } else {
                  mod_name.to_string()
                };
                quote!(#name)
              },
            )?);
          }
          code.append_all(quote! {
            impl #ty_ident #params {
              pub fn as_static_str(&self) -> &'static str {
                use #ty_ident #params ::*;
                #[allow(unreachable_patterns)]
                match self {
                  #as_str_arms
                  _ => "UnknownEvent",
                }
              }
            }

            impl From<#ty_ident #params> for &'static str {
              fn from(v: #ty_ident #params) -> Self {
                v.as_static_str()
              }
            }

            impl From<&#ty_ident #params> for &'static str {
              fn from(v: &#ty_ident #params) -> Self {
                v.as_static_str()
              }
            }
          });
        }
        _ => (),
      }
      Some((ident, code))
    }

    fn generate_types(&self) -> TokenStream {
      // Start with empty namespace.
      let mut modules = ModuleCode::new("".into());

      for ty in self.md.types.types() {
        let ty_id = ty.id();
        let ty = ty.ty();
        let ty_path = ty.path();
        let ty_ns = ty_path.namespace();
        // Only generate type code for types with namespaces.  Basic rust types like
        // `Result` and `Option` have no namespace.
        if let Some(ns_top) = ty_ns.first() {
          // Don't generate code for external types.
          if !self.external_modules.contains(ns_top) {
            if let Some((ident, code)) = self.gen_type(ty_id, ty) {
              modules.add_type(ty_ns, ident, code);
            }
          }
        }
      }

      modules.gen()
    }

    pub fn generate(self) -> TokenStream {
      let mut call_fields = TokenStream::new();
      let mut query_fields = TokenStream::new();

      // Generate module code.
      let modules: Vec<_> = self
        .md
        .pallets
        .iter()
        .map(|m| {
          let (ident, code) = self.gen_module(m);
          call_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::CallApi<'api> {
              api::#ident::CallApi::from(self.api)
            }
          });
          query_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::QueryApi<'api> {
              api::#ident::QueryApi {
                api: self.api,
                at: self.at,
              }
            }
          });

          code
        })
        .collect();

      let types_code = self.generate_types();

      let metadata_bytes = self.md.encode();
      let call_ty = &self.call;
      let event_ty = &self.event;
      quote! {
        use ::codec::Decode;

        pub const API_METADATA_BYTES: &'static [u8] = &[ #(#metadata_bytes,)* ];
        ::lazy_static::lazy_static! {
            pub static ref API_METADATA: ::polymesh_api_client::frame_metadata::v14::RuntimeMetadataV14 =
              ::polymesh_api_client::frame_metadata::v14::RuntimeMetadataV14::decode(&mut &API_METADATA_BYTES[..])
                  .expect("Shouldn't be able to fail");
        }

        #[allow(dead_code, unused_imports, non_camel_case_types)]
        pub mod types {
          use super::WrappedCall;
          #types_code
        }

        #[allow(dead_code, unused_imports, non_camel_case_types)]
        pub mod api {
          use super::types;
          use super::types::*;
          use super::WrappedCall;

          #( #modules )*
        }

        #[derive(Debug)]
        pub struct Api {
          client: ::polymesh_api_client::Client,
        }

        impl Api {
          pub async fn new(url: &str) -> ::polymesh_api_client::error::Result<Self> {
            Ok(Self {
              client: ::polymesh_api_client::Client::new(url).await?
            })
          }

          pub fn call(&self) -> CallApi {
            CallApi { api: self }
          }

          pub fn query(&self) -> QueryApi {
            QueryApi { api: self, at: None }
          }

          pub fn query_at(&self, block: ::polymesh_api_client::BlockHash) -> QueryApi {
            QueryApi { api: self, at: Some(block) }
          }

          pub fn wrap_call(&self, call: types::#call_ty) -> ::polymesh_api_client::Result<WrappedCall> {
            Ok(WrappedCall::new(self, call))
          }
        }

        #[async_trait::async_trait]
        impl ::polymesh_api_client::ChainApi for Api {
          type RuntimeCall = types::#call_ty;
          type RuntimeEvent = types::#event_ty;

          async fn get_nonce(&self, account: ::polymesh_api_client::AccountId) -> ::polymesh_api_client::Result<u32> {
            let info = self.query().system().account(account).await?;
            Ok(info.nonce)
          }

          async fn block_events(&self, block: Option<::polymesh_api_client::BlockHash>) -> ::polymesh_api_client::Result<Vec<::polymesh_api_client::EventRecord<Self::RuntimeEvent>>> {
            let system = match block {
              Some(block) => self.query_at(block).system(),
              None => self.query().system(),
            };
            Ok(system.events().await?)
          }

          fn client(&self) -> &::polymesh_api_client::Client {
            &self.client
          }
        }

        #[derive(Clone, Debug)]
        pub struct CallApi<'api> {
          api: &'api Api,
        }

        impl<'api> CallApi<'api> {
          #call_fields
        }

        pub type WrappedCall<'api> = ::polymesh_api_client::Call<'api, Api>;

        impl<'api> From<WrappedCall<'api>> for types::#call_ty {
          fn from(wrapped: WrappedCall<'api>) -> Self {
            wrapped.into_runtime_call()
          }
        }

        impl<'api> From<&WrappedCall<'api>> for types::#call_ty {
          fn from(wrapped: &WrappedCall<'api>) -> Self {
            wrapped.runtime_call().clone()
          }
        }

        #[derive(Clone, Debug)]
        pub struct QueryApi<'api> {
          api: &'api Api,
          at: Option<::polymesh_api_client::BlockHash>,
        }

        impl<'api> QueryApi<'api> {
          #query_fields
        }
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

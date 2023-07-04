use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

use heck::ToSnakeCase;

use indexmap::IndexMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use codec::Decode;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};

fn segments_ident(segments: &[String], import_types: bool) -> TokenStream {
  let idents: Vec<_> = segments.into_iter().map(|s| format_ident!("{s}")).collect();
  if import_types && idents.len() > 1 {
    quote! {
      types::#(#idents)::*
    }
  } else {
    quote! {
      #(#idents)::*
    }
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
    pallet_types: HashMap<u32, (String, String)>,
    max_error_size: usize,
    rename_types: HashMap<String, TokenStream>,
    ord_types: HashSet<String>,
    custom_derives: HashMap<String, TokenStream>,
    runtime_namespace: Vec<String>,
    call: TokenStream,
    event: TokenStream,
    v2_weights: bool,
    api_interface: TokenStream,
  }

  impl Generator {
    fn new(md: RuntimeMetadataV14) -> Self {
      // Detect the chain runtime path.
      let runtime_ty = md.types.resolve(md.ty.id()).unwrap();
      let runtime_namespace = runtime_ty.path().namespace();
      #[cfg(feature = "ink")]
      let api_interface = quote!(::polymesh_api_ink);
      #[cfg(not(feature = "ink"))]
      let api_interface = quote!(::polymesh_api_client);

      let call = quote! { runtime::RuntimeCall };
      let event = quote! { runtime::RuntimeEvent };
      let external_modules = HashSet::from_iter(["sp_version", "sp_weights"].iter().map(|t| t.to_string()));
      let rename_types = HashMap::from_iter(
        [
          (
            "sp_core::crypto::AccountId32",
            quote!(#api_interface::AccountId),
          ),
          (
            "polymesh_primitives::identity_id::IdentityId",
            quote!(#api_interface::IdentityId),
          ),
          (
            "sp_runtime::multiaddress::MultiAddress",
            quote!(#api_interface::MultiAddress),
          ),
          #[cfg(not(feature = "ink"))]
          ("sp_runtime::generic::era::Era", quote!(#api_interface::Era)),
          (
            "sp_arithmetic::per_things::Perbill",
            quote!(#api_interface::per_things::Perbill),
          ),
          (
            "sp_arithmetic::per_things::Permill",
            quote!(#api_interface::per_things::Permill),
          ),
          (
            "sp_arithmetic::per_things::PerU16",
            quote!(#api_interface::per_things::PerU16),
          ),
          (
            "sp_arithmetic::per_things::Percent",
            quote!(#api_interface::per_things::Percent),
          ),
          ("BTreeSet", quote!(::alloc::collections::BTreeSet)),
          ("BTreeMap", quote!(::alloc::collections::BTreeMap)),
          ("String", quote!(::alloc::string::String)),
          ("Vec", quote!(::alloc::vec::Vec)),
          (
            "types::frame_support::storage::weak_bounded_vec::WeakBoundedVec",
            quote!(::alloc::vec::Vec),
          ),
          (
            "types::frame_support::storage::bounded_vec::BoundedVec",
            quote!(::alloc::vec::Vec),
          ),
          (
            "types::frame_system::EventRecord",
            quote!(#api_interface::EventRecord),
          ),
          (
            "sp_weights::OldWeight",
            quote!(#api_interface::sp_weights::OldWeight),
          ),
          (
            "sp_weights::Weight",
            quote!(#api_interface::sp_weights::Weight),
          ),
          (
            "sp_weights::weight_v2::Weight",
            quote!(#api_interface::sp_weights::Weight),
          ),
        ]
        .into_iter()
        .map(|(name, code)| (name.to_string(), code)),
      );
      let ink_derives = quote! {
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadLayout))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::PackedLayout))]
        #[cfg_attr(all(feature = "ink", feature = "std"), derive(::ink_storage::traits::StorageLayout))]
      };
      let ink_enum_derives = quote! {
        #[derive(Copy)]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadLayout))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::PackedLayout))]
        #[cfg_attr(all(feature = "ink", feature = "std"), derive(::ink_storage::traits::StorageLayout))]
      };
      let ink_extra_derives = quote! {
        #[derive(Default)]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadAllocate))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadLayout))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::PackedLayout))]
        #[cfg_attr(all(feature = "ink", feature = "std"), derive(::ink_storage::traits::StorageLayout))]
      };
      let ink_id_derives = quote! {
        #[derive(Copy, Default)]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadAllocate))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::SpreadLayout))]
        #[cfg_attr(feature = "ink", derive(::ink_storage::traits::PackedLayout))]
        #[cfg_attr(all(feature = "ink", feature = "std"), derive(::ink_storage::traits::StorageLayout))]
      };
      let custom_derives = HashMap::from_iter(
        [
          // Asset types.
          ("AssetName", &ink_extra_derives),
          ("AssetType", &ink_enum_derives),
          ("NonFungibleType", &ink_enum_derives),
          ("AssetIdentifier", &ink_enum_derives),
          ("CustomAssetTypeId", &ink_id_derives),
          ("FundingRoundName", &ink_extra_derives),
          ("Ticker", &ink_id_derives),
          // Portfolio
          ("PortfolioId", &ink_enum_derives),
          ("PortfolioKind", &ink_enum_derives),
          ("PortfolioNumber", &ink_id_derives),
          ("MovePortfolioItem", &ink_derives),
          ("Memo", &ink_derives),
          // Settlement types.
          ("VenueId", &ink_id_derives),
          ("VenueDetails", &ink_extra_derives),
          ("VenueType", &ink_enum_derives),
          ("Leg", &ink_derives),
          ("LegId", &ink_id_derives),
          ("InstructionId", &ink_id_derives),
          ("AffirmationStatus", &ink_enum_derives),
          ("InstructionStatus", &ink_enum_derives),
          ("LegStatus", &ink_enum_derives),
          ("SettlementType", &ink_enum_derives),
        ]
        .into_iter()
        .map(|(name, code)| (name.to_string(), code.clone())),
      );

      let mut gen = Self {
        runtime_namespace: runtime_namespace.iter().cloned().collect(),
        md,
        external_modules,
        pallet_types: HashMap::new(),
        max_error_size: 4,
        rename_types,
        ord_types: Default::default(),
        custom_derives,
        call,
        event,
        v2_weights: false,
        api_interface,
      };
      // Try a limited number of times to mark all types needing the `Ord` type.
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

      gen.detect_v2_weights();

      // Rename pallet types.
      gen.rename_pallet_types();

      gen
    }

    fn rename_pallet_type(&mut self, id: u32, p_name: &str, kind: &str) {
      let ty = self.md.types.resolve(id).unwrap();
      let path = ty.path();
      let mut segments: Vec<_> = path.segments().into_iter().cloned().collect();
      let old_name = segments.join("::");
      let new_name = format!("{}{}", p_name, kind);
      if let Some(last) = segments.last_mut() {
        *last = new_name.clone();
      }
      let new_ident = segments_ident(&segments, false);
      self.rename_types.insert(old_name, new_ident);
      self.pallet_types.insert(id, (p_name.to_string(), new_name));
    }

    // Rename pallet types Call/Event/Error.
    fn rename_pallet_types(&mut self) {
      // Collect pallet type ids.
      let types: Vec<_> = self.md.pallets.iter().map(|p| {
        (p.name.to_string(), p.calls.clone(), p.event.clone(), p.error.clone())
      }).collect();
      for (p_name, call, event, error) in types {
        if let Some(c) = call {
          self.rename_pallet_type(c.ty.id(), &p_name, "Call");
        }
        if let Some(e) = event {
          self.rename_pallet_type(e.ty.id(), &p_name, "Event");
        }
        if let Some(e) = error {
          self.rename_pallet_type(e.ty.id(), &p_name, "Error");
        }
      }
    }

    // Detect if chain is using V2 Weights.
    fn detect_v2_weights(&mut self) {
      for ty in self.md.types.types() {
        let id = ty.id();
        let full_name = self.id_to_full_name(id).unwrap_or_default();
        if full_name == "frame_support::dispatch::DispatchInfo" {
          self.v2_weights = true;
          return;
        }
      }
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

    fn type_name(&self, id: u32, compact_wrap: bool, import_types: bool) -> Option<TokenStream> {
      let mut scope = TypeParameters::default();
      self.type_name_scoped(id, &mut scope, compact_wrap, import_types)
    }

    fn type_name_scoped(
      &self,
      id: u32,
      scope: &mut TypeParameters,
      compact_wrap: bool,
      import_types: bool,
    ) -> Option<TokenStream> {
      if let Some(scope_type) = scope.get_param(id) {
        return Some(scope_type);
      }
      let ty = self.md.types.resolve(id)?;
      let path = ty.path();
      let (type_ident, is_btree) = match self.is_runtime_type(path) {
        Some(name) => {
          // Remap runtime types to namespace `runtime`.
          let ident = format_ident!("{name}");
          (quote!(runtime::#ident), false)
        },
        None => {
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
            .unwrap_or_else(|| segments_ident(segments, import_types));
          (type_ident, is_btree)
        },
      };

      match ty.type_def() {
        TypeDef::Sequence(ty) => {
          return self
            .type_name_scoped(ty.type_param().id(), scope, true, import_types)
            .map(|elem_ty| {
              quote! { ::alloc::vec::Vec<#elem_ty> }
            });
        }
        TypeDef::Array(ty) => {
          let len = ty.len() as usize;
          return self
            .type_name_scoped(ty.type_param().id(), scope, true, import_types)
            .map(|elem_ty| {
              quote! { [#elem_ty; #len] }
            });
        }
        TypeDef::Tuple(ty) => {
          let fields = ty
            .fields()
            .into_iter()
            .filter_map(|field| self.type_name_scoped(field.id(), scope, true, import_types))
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
              Str => return Some(quote!(::alloc::string::String)),
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
            .type_name_scoped(ty.type_param().id(), scope, true, import_types)
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
            .map(|ty| self.type_name_scoped(ty.id(), scope, true, import_types))
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
      let api_interface = &self.api_interface;
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
          .type_name(key.id(), false, true)
          .expect("Missing Storage key type");
        keys.append_all(quote! {#key_ident: #type_name,});
        hashing.append_all(match hasher {
          StorageHasher::Blake2_128 => quote! {
            buf.extend(#api_interface::hashing::blake2_128(&#key_ident.encode()));
          },
          StorageHasher::Blake2_256 => quote! {
            buf.extend(#api_interface::hashing::blake2_256(&#key_ident.encode()));
          },
          StorageHasher::Blake2_128Concat => quote! {
            let key = #key_ident.encode();
            buf.extend(#api_interface::hashing::blake2_128(&key));
            buf.extend(key.into_iter());
          },
          StorageHasher::Twox128 => quote! {
            buf.extend(#api_interface::hashing::twox_128(&#key_ident.encode()));
          },
          StorageHasher::Twox256 => quote! {
            buf.extend(#api_interface::hashing::twox_256(&#key_ident.encode()));
          },
          StorageHasher::Twox64Concat => quote! {
            let key = #key_ident.encode();
            buf.extend(#api_interface::hashing::twox_64(&key));
            buf.extend(key.into_iter());
          },
          StorageHasher::Identity => quote! {
            buf.extend(#key_ident.encode());
          },
        });
      }
      let value_ty = if mod_prefix == "System" && storage_name == "Events" {
        let event_ty = &self.event;
        quote!(::alloc::vec::Vec<#api_interface::EventRecord<types::#event_ty>>)
      } else {
        self.type_name(value_ty, false, false).unwrap()
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
          #[cfg(not(feature = "ink"))]
          pub async fn #storage_ident(&self, #keys) -> ::polymesh_api_client::error::Result<#return_ty> {
            use ::codec::Encode;
            let mut buf = ::alloc::vec::Vec::with_capacity(512);
            buf.extend([#(#key_prefix,)*]);
            #hashing
            let key = ::polymesh_api_client::StorageKey(buf);
            let value = self.api.client.get_storage_by_key(key, self.at).await?;
            #return_value
          }

          #(#[doc = #docs])*
          #[cfg(feature = "ink")]
          pub fn #storage_ident(&self, #keys) -> ::polymesh_api_ink::error::Result<#return_ty> {
            use ::codec::Encode;
            let mut buf = ::alloc::vec::Vec::with_capacity(512);
            buf.extend([#(#key_prefix,)*]);
            #hashing
            let value = self.api.read_storage(buf)?;
            #return_value
          }
        }
      } else {
        quote! {
          #(#[doc = #docs])*
          #[cfg(not(feature = "ink"))]
          pub async fn #storage_ident(&self) -> ::polymesh_api_client::error::Result<#return_ty> {
            let key = ::polymesh_api_client::StorageKey(vec![#(#key_prefix,)*]);
            let value = self.api.client.get_storage_by_key(key, self.at).await?;
            #return_value
          }

          #(#[doc = #docs])*
          #[cfg(feature = "ink")]
          pub fn #storage_ident(&self) -> ::polymesh_api_ink::error::Result<#return_ty> {
            let value = self.api.read_storage(::alloc::vec![#(#key_prefix,)*])?;
            #return_value
          }
        }
      }
    }

    fn gen_func(
      &self,
      mod_name: &str,
      mod_idx: u8,
      mod_call_ty: u32,
      md: &Variant<PortableForm>,
    ) -> TokenStream {
      let mod_call_ident = format_ident!("{mod_name}");
      let mod_call = self.type_name(mod_call_ty, false, true).unwrap();
      let func_name = md.name();
      let func_idx = md.index();
      let func_ident = format_ident!("{}", func_name.to_snake_case());

      let mut fields = TokenStream::new();
      let mut field_names = TokenStream::new();
      let mut fields_encode = TokenStream::new();
      for (idx, field) in md.fields().iter().enumerate() {
        let name = field
          .name()
          .map(|n| format_ident!("{n}"))
          .unwrap_or_else(|| format_ident!("param_{idx}"));
        let type_name = self
          .type_name(field.ty().id(), false, true)
          .expect("Missing Extrinsic param type");
        fields.append_all(quote! {#name: #type_name,});
        if Self::is_boxed(field) {
          field_names.append_all(quote! {#name: ::alloc::boxed::Box::new(#name),});
        } else {
          field_names.append_all(quote! {#name,});
        }
        fields_encode.append_all(quote! {
          #name.encode_to(&mut buf);
        });
      }

      let docs = md.docs();
      let call_ty = &self.call;
      if md.fields().len() > 0 {
        quote! {
          #(#[doc = #docs])*
          #[cfg(not(feature = "ink"))]
          pub fn #func_ident(&self, #fields) -> ::polymesh_api_client::error::Result<super::super::WrappedCall<'api>> {
            self.api.wrap_call(#call_ty::#mod_call_ident(types::#mod_call::#func_ident { #field_names }))
          }

          #(#[doc = #docs])*
          #[cfg(feature = "ink")]
          pub fn #func_ident(&self, #fields) -> super::super::WrappedCall {
            use ::codec::Encode;
            let mut buf = ::alloc::vec![#mod_idx, #func_idx];
            #fields_encode
            self.api.wrap_call(buf)
          }
        }
      } else {
        quote! {
          #(#[doc = #docs])*
          #[cfg(not(feature = "ink"))]
          pub fn #func_ident(&self) -> ::polymesh_api_client::error::Result<super::super::WrappedCall<'api>> {
            self.api.wrap_call(#call_ty::#mod_call_ident(types::#mod_call::#func_ident))
          }

          #(#[doc = #docs])*
          #[cfg(feature = "ink")]
          pub fn #func_ident(&self) -> super::super::WrappedCall {
            self.api.wrap_call(::alloc::vec![#mod_idx, #func_idx])
          }
        }
      }
    }

    fn gen_module(
      &self,
      md: &frame_metadata::v14::PalletMetadata<PortableForm>,
    ) -> (Ident, Ident, Ident, TokenStream) {
      let mod_idx = md.index;
      let mod_name = &md.name;
      let mod_call_api = format_ident!("{}CallApi", mod_name);
      let mod_query_api = format_ident!("{}QueryApi", mod_name);
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

          #[derive(Clone)]
          pub struct #mod_call_api<'api> {
            api: &'api super::super::Api,
          }

          impl<'api> #mod_call_api<'api> {
            #call_fields
          }

          impl<'api> From<&'api super::super::Api> for #mod_call_api<'api> {
            fn from(api: &'api super::super::Api) -> Self {
              Self { api }
            }
          }

          #[derive(Clone)]
          pub struct #mod_query_api<'api> {
            pub(crate) api: &'api super::super::Api,
            #[cfg(not(feature = "ink"))]
            pub(crate) at: Option<::polymesh_api_client::BlockHash>,
          }

          impl<'api> #mod_query_api<'api> {
            #query_fields
          }
        }
      };
      (mod_ident, mod_call_api, mod_query_api, code)
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
        let mut field_ty = self.type_name_scoped(field.ty().id(), scope, false, false)?;
        if Self::is_boxed(field) {
          field_ty = quote!(::alloc::boxed::Box<#field_ty>);
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
        let mut field_ty = self.type_name_scoped(field.ty().id(), scope, false, false)?;
        if Self::is_boxed(field) {
          field_ty = quote!(::alloc::boxed::Box<#field_ty>);
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

    fn gen_enum_as_static_str(
      &self,
      ty_ident: &Ident,
      params: &TokenStream,
      ty: &Type<PortableForm>,
      prefix: Option<&str>,
    ) -> Option<TokenStream> {
      let mut as_str_arms = TokenStream::new();
      let mut as_docs_arms = TokenStream::new();
      match (prefix, ty.type_def()) {
        (None, TypeDef::Variant(enum_ty)) => {
          for variant in enum_ty.variants() {
            let top_name = variant.name();
            let top_ident = format_ident!("{}", top_name);
            let fields = variant.fields().len();
            if fields == 1 {
              as_str_arms.append_all(quote! {
                Self::#top_ident(val) => {
                  val.as_static_str()
                },
              });
              as_docs_arms.append_all(quote! {
                Self::#top_ident(val) => {
                  val.as_docs()
                },
              });
            } else {
              as_str_arms.append_all(quote! {
                Self::#top_ident(_) => {
                  #top_name
                },
              });
              as_docs_arms.append_all(quote! {
                Self::#top_ident(_) => {
                  &[""]
                },
              });
            }
          }
        }
        (Some(prefix), TypeDef::Variant(enum_ty)) => {
          for variant in enum_ty.variants() {
            let var_name = variant.name();
            let var_ident = format_ident!("{}", var_name);
            let mut docs = variant.docs().to_vec();
            if docs.len() == 0 {
              docs.push("".to_string());
            }
            let as_str_name = format!("{}.{}", prefix, var_name);
            let match_fields = self.gen_enum_match_fields(variant.fields());
            as_str_arms.append_all(quote! {
              Self:: #var_ident #match_fields => {
                #as_str_name
              },
            });
            as_docs_arms.append_all(quote! {
              Self:: #var_ident #match_fields => {
                &[#(#docs,)*]
              },
            });
          }
        }
        _ => {
          return None;
        }
      };
      Some(quote! {
        impl #params #ty_ident #params {
          pub fn as_static_str(&self) -> &'static str {
            #[allow(unreachable_patterns)]
            match self {
              #as_str_arms
              _ => "Unknown",
            }
          }
        }

        #[cfg(not(feature = "ink"))]
        impl #params ::polymesh_api_client::EnumInfo for #ty_ident #params {
          fn as_name(&self) -> &'static str {
            self.as_static_str()
          }

          fn as_docs(&self) -> &'static [&'static str] {
            #[allow(unreachable_patterns)]
            match self {
              #as_docs_arms
              _ => &[""],
            }
          }
        }

        impl #params From<#ty_ident #params> for &'static str {
          fn from(v: #ty_ident #params) -> Self {
            v.as_static_str()
          }
        }

        impl #params From<&#ty_ident #params> for &'static str {
          fn from(v: &#ty_ident #params) -> Self {
            v.as_static_str()
          }
        }
      })
    }

    fn is_runtime_type(&self, path: &Path<PortableForm>) -> Option<String> {
      if self.runtime_namespace == path.namespace() {
        let ident = path.ident();
        match ident.as_deref() {
          Some("Event") => Some("RuntimeEvent".into()),
          Some("Call") => Some("RuntimeCall".into()),
          Some(name) => Some(name.into()),
          _ => None,
        }
      } else {
        None
      }
    }

    fn gen_module_error(&self, _id: u32, ty: &Type<PortableForm>, ident: &str) -> Option<TokenStream> {
      let ty_ident = format_ident!("{ident}");
      let mut scope = TypeParameters::new(ty);

      let mut variants = TokenStream::new();
      let mut as_str_arms = TokenStream::new();
      let mut as_docs_arms = TokenStream::new();
      for p in &self.md.pallets {
        let idx = p.index;
        let mod_ident = format_ident!("{}", p.name);
        let error_ty = p.error.as_ref().and_then(|e| {
          self
            .type_name_scoped(e.ty.id(), &mut scope, false, false)
            .map(|ident| quote! { (#ident) })
        });
        if let Some(error_ty) = error_ty {
          variants.append_all(quote! {
            #[codec(index = #idx)]
            #mod_ident #error_ty,
          });
          as_str_arms.append_all(quote! {
            RuntimeError:: #mod_ident(err) => err.as_static_str(),
          });
          as_docs_arms.append_all(quote! {
            RuntimeError:: #mod_ident(err) => err.as_docs(),
          });
        }
      }

      let docs = ty.docs();
      let max_error_size = self.max_error_size + 1;
      let code = quote! {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[derive(::codec::Encode, ::codec::Decode)]
        #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum RuntimeError {
          #variants
        }

        impl RuntimeError {
          pub fn as_static_str(&self) -> &'static str {
            match self {
              #as_str_arms
            }
          }
        }

        impl From<RuntimeError> for &'static str {
          fn from(v: RuntimeError) -> Self {
            v.as_static_str()
          }
        }

        impl From<&RuntimeError> for &'static str {
          fn from(v: &RuntimeError) -> Self {
            v.as_static_str()
          }
        }

        #[cfg(not(feature = "ink"))]
        impl ::polymesh_api_client::EnumInfo for RuntimeError {
          fn as_name(&self) -> &'static str {
            self.as_static_str()
          }

          fn as_docs(&self) -> &'static [&'static str] {
            match self {
              #as_docs_arms
            }
          }
        }

        #(#[doc = #docs])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #ty_ident(pub RuntimeError);

        impl ::codec::Encode for #ty_ident {
          fn encode_to<T: ::codec::Output + ?Sized>(&self, output: &mut T) {
            let mut raw = self.0.encode();
            raw.resize(#max_error_size, 0);
            output.write(raw.as_slice());
          }
        }

        impl ::codec::Decode for #ty_ident {
          fn decode<I: ::codec::Input>(input: &mut I) -> Result<Self, ::codec::Error> {
            let raw: [u8; #max_error_size] = ::codec::Decode::decode(input)?;
            Ok(Self(RuntimeError::decode(&mut &raw[..])?))
          }
        }

        impl #ty_ident {
          pub fn as_static_str(&self) -> &'static str {
            self.0.as_static_str()
          }
        }

        impl From<#ty_ident> for &'static str {
          fn from(v: #ty_ident) -> Self {
            v.as_static_str()
          }
        }

        impl From<&#ty_ident> for &'static str {
          fn from(v: &#ty_ident) -> Self {
            v.as_static_str()
          }
        }

        #[cfg(not(feature = "ink"))]
        impl ::polymesh_api_client::EnumInfo for #ty_ident {
          fn as_name(&self) -> &'static str {
            self.as_static_str()
          }

          fn as_docs(&self) -> &'static [&'static str] {
            self.0.as_docs()
          }
        }
      };
      Some(code)
    }

    fn gen_dispatch_error(
      &self,
      _id: u32,
      ty: &Type<PortableForm>,
      ident: &str,
    ) -> Option<TokenStream> {
      let ty_ident = format_ident!("{ident}");
      let mut scope = TypeParameters::new(ty);

      let mut variants = TokenStream::new();
      let mut as_str_arms = TokenStream::new();
      let mut as_docs_arms = TokenStream::new();
      for p in &self.md.pallets {
        let idx = p.index;
        let mod_ident = format_ident!("{}", p.name);
        let error_ty = p.error.as_ref().and_then(|e| {
          self
            .type_name_scoped(e.ty.id(), &mut scope, false, false)
            .map(|ident| quote! { (#ident) })
        });
        if let Some(error_ty) = error_ty {
          variants.append_all(quote! {
            #[codec(index = #idx)]
            #mod_ident #error_ty,
          });
          as_str_arms.append_all(quote! {
            RuntimeError:: #mod_ident(err) => err.as_static_str(),
          });
          as_docs_arms.append_all(quote! {
            RuntimeError:: #mod_ident(err) => err.as_docs(),
          });
        }
      }

      let docs = ty.docs();
      let code = quote! {
        #(#[doc = #docs])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[derive(::codec::Encode, ::codec::Decode)]
        #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub enum #ty_ident {
          Other,
          CannotLookup,
          BadOrigin,
          Module(ModuleError),
          ConsumerRemaining,
          NoProviders,
          TooManyConsumers,
          Token(TokenError),
          Arithmetic(sp_arithmetic::ArithmeticError),
        }

        impl #ty_ident {
          pub fn as_static_str(&self) -> &'static str {
            match self {
              Self::Other => "Other",
              Self::CannotLookup => "CannotLookup",
              Self::BadOrigin => "BadOrigin",
              Self::Module(err) => err.as_static_str(),
              Self::ConsumerRemaining => "ConsumerRemaining",
              Self::NoProviders => "NoProviders",
              Self::TooManyConsumers => "TooManyConsumers",
              Self::Token(err) => {
                match err {
                  TokenError::NoFunds => "Token::NoFunds",
                  TokenError::WouldDie => "Token::WouldDie",
                  TokenError::BelowMinimum => "Token::BelowMinimum",
                  TokenError::CannotCreate => "Token::CannotCreate",
                  TokenError::UnknownAsset => "Token::UnknownAsset",
                  TokenError::Frozen => "Token::Frozen",
                  TokenError::Unsupported => "Token::Unsupported",
                }
              },
              Self::Arithmetic(err) => {
                match err {
                  sp_arithmetic::ArithmeticError::Underflow => "Arithmetic::Underflow",
                  sp_arithmetic::ArithmeticError::Overflow => "Arithmetic::Overflow",
                  sp_arithmetic::ArithmeticError::DivisionByZero => "Arithmetic::DivisionByZero",
                }
              },
            }
          }
        }

        impl From<#ty_ident> for &'static str {
          fn from(v: #ty_ident) -> Self {
            v.as_static_str()
          }
        }

        impl From<&#ty_ident> for &'static str {
          fn from(v: &#ty_ident) -> Self {
            v.as_static_str()
          }
        }

        #[cfg(not(feature = "ink"))]
        impl ::polymesh_api_client::EnumInfo for #ty_ident {
          fn as_name(&self) -> &'static str {
            self.as_static_str()
          }

          fn as_docs(&self) -> &'static [&'static str] {
            match self {
              Self::Other => &["Some error occurred."],
              Self::CannotLookup => &["Failed to lookup some data."],
              Self::BadOrigin => &["A bad origin."],
              Self::Module(err) => err.as_docs(),
              Self::ConsumerRemaining => &["At least one consumer is remaining so the account cannot be destroyed."],
              Self::NoProviders => &["There are no providers so the account cannot be created."],
              Self::TooManyConsumers => &["There are too many consumers so the account cannot be created."],
              Self::Token(err) => {
                match err {
                  TokenError::NoFunds => &["Funds are unavailable."],
                  TokenError::WouldDie => &["Account that must exist would die."],
                  TokenError::BelowMinimum => &["Account cannot exist with the funds that would be given."],
                  TokenError::CannotCreate => &["Account cannot be created."],
                  TokenError::UnknownAsset => &["The asset in question is unknown."],
                  TokenError::Frozen => &["Funds exist but are frozen."],
                  TokenError::Unsupported => &["Operation is not supported by the asset."],
                }
              },
              Self::Arithmetic(err) => {
                match err {
                  sp_arithmetic::ArithmeticError::Underflow => &["Arithmetic underflow"],
                  sp_arithmetic::ArithmeticError::Overflow => &["Arithmetic overflow"],
                  sp_arithmetic::ArithmeticError::DivisionByZero => &["Arithmetic divide by zero"],
                }
              },
            }
          }
        }
      };
      Some(code)
    }

    fn gen_type(&self, id: u32, ty: &Type<PortableForm>, ident: &str, is_runtime_type: bool) -> Option<TokenStream> {
      let full_name = self.id_to_full_name(id)?;
      if full_name == "sp_runtime::ModuleError" {
        return self.gen_module_error(id, ty, ident);
      }
      if full_name == "sp_runtime::DispatchError" {
        return self.gen_dispatch_error(id, ty, ident);
      }
      let (pallet_name, ident) = match self.pallet_types.get(&id) {
        Some((pallet_name, ident)) => (Some(pallet_name), ident.as_str()),
        None => (None, ident),
      };
      let ty_ident = format_ident!("{ident}");

      let mut scope = TypeParameters::new(ty);
      let derive_ord = if self.ord_types.contains(&full_name) {
        quote! {
          #[derive(PartialOrd, Ord)]
        }
      } else {
        quote!()
      };
      let custom_derive = self
        .custom_derives
        .get(ident)
        .cloned()
        .unwrap_or_else(|| quote!());

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
                #custom_derive
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
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
                #custom_derive
                #[derive(::codec::Encode, ::codec::Decode)]
                #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
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
              #custom_derive
              #[derive(::codec::Encode, ::codec::Decode)]
              #[cfg_attr(all(feature = "std", feature = "type_info"), derive(::scale_info::TypeInfo))]
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

      // Special handling for pallet types.
      if let Some(pallet_name) = pallet_name {
        if let Some(as_static_str) =
          self.gen_enum_as_static_str(&ty_ident, &params, ty, Some(pallet_name))
        {
          code.append_all(as_static_str);
        }
      }

      // For runtime types generate enum -> static string helpers.
      if is_runtime_type && (ident == "RuntimeCall" || ident == "RuntimeEvent") {
        if let Some(as_static_str) = self.gen_enum_as_static_str(&ty_ident, &params, ty, None) {
          code.append_all(as_static_str);
        }
      }
      Some(code)
    }

    fn generate_types(&self) -> TokenStream {
      // Start with empty namespace.
      let mut modules = ModuleCode::new("".into());
      let runtime_ns = [String::from("runtime")];

      for ty in self.md.types.types() {
        let ty_id = ty.id();
        let ty = ty.ty();
        let ty_path = ty.path();
        let mut ty_ns = ty_path.namespace();
        // Only generate type code for types with namespaces.  Basic rust types like
        // `Result` and `Option` have no namespace.
        if let Some(ns_top) = ty_ns.first() {
          // Don't generate code for external types.
          if !self.external_modules.contains(ns_top) {
            let (ident, is_runtime_type) = match self.is_runtime_type(ty_path) {
              Some(name) => {
                ty_ns = &runtime_ns;
                (name, true)
              },
              None => (ty_path.ident().unwrap(), false),
            };

            if let Some(code) = self.gen_type(ty_id, ty, &ident, is_runtime_type) {
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
          let (ident, call_api, query_api, code) = self.gen_module(m);
          call_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::#call_api<'api> {
              api::#ident::#call_api::from(self.api)
            }
          });
          query_fields.append_all(quote! {
            pub fn #ident(&self) -> api::#ident::#query_api<'api> {
              api::#ident::#query_api {
                api: self.api,
                #[cfg(not(feature = "ink"))]
                at: self.at,
              }
            }
          });

          code
        })
        .collect();

      let types_code = self.generate_types();

      let dispatch_info = if self.v2_weights {
        quote! { frame_support::dispatch::DispatchInfo }
      } else {
        quote! { frame_support::weights::DispatchInfo }
      };

      let call_ty = &self.call;
      let event_ty = &self.event;
      quote! {
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

        pub struct Api {
          #[cfg(not(feature = "ink"))]
          client: ::polymesh_api_client::Client,
        }

        impl Api {
          #[cfg(feature = "ink")]
          pub fn new() -> Self {
            Self {}
          }

          #[cfg(feature = "ink")]
          pub fn runtime(&self) -> ::polymesh_api_ink::extension::PolymeshRuntimeInstance {
            ::polymesh_api_ink::extension::new_instance()
          }

          #[cfg(feature = "ink")]
          pub fn read_storage<T: ::codec::Decode>(&self, key: ::alloc::vec::Vec<u8>) -> ::polymesh_api_ink::error::Result<Option<T>> {
            let runtime = self.runtime();
            let value = runtime.read_storage(key.into())?
              .map(|data| T::decode(&mut data.as_slice()))
              .transpose()?;
            Ok(value)
          }

          #[cfg(not(feature = "ink"))]
          pub async fn new(url: &str) -> ::polymesh_api_client::error::Result<Self> {
            Ok(Self {
              client: ::polymesh_api_client::Client::new(url).await?
            })
          }

          pub fn call(&self) -> CallApi {
            CallApi { api: self }
          }

          #[cfg(not(feature = "ink"))]
          pub fn query(&self) -> QueryApi {
            QueryApi { api: self, at: None }
          }

          #[cfg(feature = "ink")]
          pub fn query(&self) -> QueryApi {
            QueryApi { api: self }
          }

          #[cfg(not(feature = "ink"))]
          pub fn query_at(&self, block: ::polymesh_api_client::BlockHash) -> QueryApi {
            QueryApi { api: self, at: Some(block) }
          }

          #[cfg(not(feature = "ink"))]
          pub fn wrap_call(&self, call: types::#call_ty) -> ::polymesh_api_client::Result<WrappedCall> {
            Ok(WrappedCall::new(self, call))
          }

          #[cfg(feature = "ink")]
          pub fn wrap_call(&self, call: ::alloc::vec::Vec<u8>) -> WrappedCall {
            WrappedCall::new(call)
          }
        }

        #[async_trait::async_trait]
        #[cfg(not(feature = "ink"))]
        impl ::polymesh_api_client::ChainApi for Api {
          type RuntimeCall = types::#call_ty;
          type RuntimeEvent = types::#event_ty;
          type DispatchInfo = types::#dispatch_info;
          type DispatchError = types::sp_runtime::DispatchError;

          async fn get_nonce(&self, account: ::polymesh_api_client::AccountId) -> ::polymesh_api_client::Result<u32> {
            let info = self.query().system().account(account).await?;
            Ok(info.nonce)
          }

          async fn block_events(&self, block: Option<::polymesh_api_client::BlockHash>) -> ::polymesh_api_client::Result<::alloc::vec::Vec<::polymesh_api_client::EventRecord<Self::RuntimeEvent>>> {
            let system = match block {
              Some(block) => self.query_at(block).system(),
              None => self.query().system(),
            };
            Ok(system.events().await?)
          }

          fn event_to_extrinsic_result(event: &::polymesh_api_client::EventRecord<Self::RuntimeEvent>) -> Option<::polymesh_api_client::ExtrinsicResult<Self>> {
            match &event.event {
              types::#event_ty::System(types::frame_system::pallet::SystemEvent::ExtrinsicSuccess { dispatch_info }) =>
                Some(::polymesh_api_client::ExtrinsicResult::Success(dispatch_info.clone())),
              types::#event_ty::System(types::frame_system::pallet::SystemEvent::ExtrinsicFailed { dispatch_info, dispatch_error }) =>
                Some(::polymesh_api_client::ExtrinsicResult::Failed(dispatch_info.clone(), dispatch_error.clone())),
              _ => None,
            }
          }

          fn client(&self) -> &::polymesh_api_client::Client {
            &self.client
          }
        }

        #[derive(Clone)]
        pub struct CallApi<'api> {
          api: &'api Api,
        }

        impl<'api> CallApi<'api> {
          #call_fields
        }

        #[cfg(not(feature = "ink"))]
        pub type WrappedCall<'api> = ::polymesh_api_client::Call<'api, Api>;
        #[cfg(feature = "ink")]
        pub type WrappedCall = ::polymesh_api_ink::Call;

        #[cfg(not(feature = "ink"))]
        impl<'api> From<WrappedCall<'api>> for types::#call_ty {
          fn from(wrapped: WrappedCall<'api>) -> Self {
            wrapped.into_runtime_call()
          }
        }

        #[cfg(not(feature = "ink"))]
        impl<'api> From<&WrappedCall<'api>> for types::#call_ty {
          fn from(wrapped: &WrappedCall<'api>) -> Self {
            wrapped.runtime_call().clone()
          }
        }

        #[derive(Clone)]
        pub struct QueryApi<'api> {
          api: &'api Api,
          #[cfg(not(feature = "ink"))]
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

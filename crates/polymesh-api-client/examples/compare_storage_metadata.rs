use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use anyhow::Result;

use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use frame_metadata::v14::*;
use scale_info::{
  form::PortableForm,
  PortableRegistry,
  TypeDef,
};
use codec::Decode;

pub struct StorageMetadata {
  pub prefix: String,
  pub entries: HashMap<String, StorageEntryMetadata<PortableForm>>,
}

impl StorageMetadata {
  pub fn from_v14(md: PalletStorageMetadata<PortableForm>) -> Self {
    Self {
      prefix: md.prefix,
      entries: md.entries.into_iter().map(|entry| (entry.name.clone(), entry)).collect(),
    }
  }
}

pub struct Metadata {
  pub types: PortableRegistry,
  pub storage: HashMap<String, StorageMetadata>,
}

impl Metadata {
  pub fn from_v14(md: RuntimeMetadataV14) -> Self {
    let mut storage_prefix = HashMap::new();
    Self {
      types: md.types,
      storage: md.pallets.into_iter().filter_map(|p| {
        p.storage.map(|s| {
          if let Some(old_pallet) = storage_prefix.insert(s.prefix.clone(), p.name.clone()) {
            log::error!("Duplicate storage prefix '{}' used by {} and {}", s.prefix, old_pallet, p.name);
          }
          (p.name.clone(), StorageMetadata::from_v14(s))
        })
      }).collect(),
    }
  }

  pub fn from_file(filename: String) -> Result<Self> {
    let mut file = File::open(&filename)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let metadata = RuntimeMetadataPrefixed::decode(&mut buf.as_slice())?;
    match metadata.1 {
      RuntimeMetadata::V14(md) => Ok(Self::from_v14(md)),
      _ => {
        panic!("Unsupported metadata version: {:?}", metadata);
      }
    }
  }

  pub fn is_types_compatible(&self, seen: &mut HashMap<(u32, u32), bool>, other: &Self, id1: u32, id2: u32) -> bool {
    let mut compatible = true;

    if let Some(comp) = seen.get(&(id1, id2)) {
      return *comp;
    }
    // Mark the type pair as seen to prevent recursive checks.
    seen.insert((id1, id2), true);

    let ty1 = self.types.resolve(id1).expect("expect type in metadata1");
    let ty2 = other.types.resolve(id2).expect("expect type in metadata2");

    // Ignore `RuntimeCall`.
    let ident1 = ty1.path().ident().unwrap_or_default();
    let ident2 = ty2.path().ident().unwrap_or_default();
    match ident1.as_str() {
      "RuntimeCall" | "RuntimeEvent" => {
        return true;
      }
      _ => (),
    }

    match (ty1.type_def(), ty2.type_def()) {
      (TypeDef::Composite(v1), TypeDef::Composite(v2)) => {
        if v1.fields().len() != v2.fields().len() {
          compatible = false;
          log::trace!("Composites have different number of fields: {v1:?} != {v2:?}");
        } else {
          for (f1, f2) in v1.fields().into_iter().zip(v2.fields().into_iter()) {
            if !self.is_types_compatible(seen, other, f1.ty().id(), f2.ty().id()) {
              compatible = false;
            }
          }
        }
      }
      (TypeDef::Variant(v1), TypeDef::Variant(v2)) => {
        let variants2: HashMap<u8, _> = v2.variants().iter().map(|v| (v.index, v)).collect();
        for variant1 in v1.variants() {
          match variants2.get(&variant1.index) {
            Some(variant2) => {
              if variant1.fields.len() != variant2.fields.len() {
                compatible = false;
                log::trace!("Enum variant has different number of fields: {variant1:?} != {variant2:?}");
              } else {
                for (f1, f2) in variant1.fields.iter().zip(variant2.fields.iter()) {
                  if !self.is_types_compatible(seen, other, f1.ty().id(), f2.ty().id()) {
                    compatible = false;
                  }
                }
              }
            }
            None => {
              compatible = false;
              log::trace!("Enum variant removed: {:?}", variant1);
            }
          }
        }
      }
      (TypeDef::Sequence(v1), TypeDef::Sequence(v2)) => {
        if !self.is_types_compatible(seen, other, v1.type_param().id(), v2.type_param().id()) {
          compatible = false;
        }
      }
      (TypeDef::Array(v1), TypeDef::Array(v2)) => {
        if v1.len() != v2.len() {
          compatible = false;
          log::trace!("Different Array lengths: {v1:?} != {v2:?}");
        }
        if !self.is_types_compatible(seen, other, v1.type_param().id(), v2.type_param().id()) {
          compatible = false;
        }
      }
      (TypeDef::Tuple(v1), TypeDef::Tuple(v2)) => {
        if v1.fields().len() != v2.fields().len() {
          compatible = false;
          log::trace!("Tuples have different number of fields: {v1:?} != {v2:?}");
        }
        for (f1, f2) in v1.fields().into_iter().zip(v2.fields().into_iter()) {
          if !self.is_types_compatible(seen, other, f1.id(), f2.id()) {
            compatible = false;
          }
        }
      }
      (TypeDef::Primitive(v1), TypeDef::Primitive(v2)) => {
        if v1 != v2 {
          compatible = false;
        }
      }
      (TypeDef::Compact(v1), TypeDef::Compact(v2)) => {
        if !self.is_types_compatible(seen, other, v1.type_param().id(), v2.type_param().id()) {
          compatible = false;
        }
      }
      (TypeDef::BitSequence(v1), TypeDef::BitSequence(v2)) => {
        if !self.is_types_compatible(seen, other, v1.bit_order_type().id(), v2.bit_order_type().id()) {
          compatible = false;
        }
        if !self.is_types_compatible(seen, other, v1.bit_store_type().id(), v2.bit_store_type().id()) {
          compatible = false;
        }
      }
      _ => {
        compatible = false;
        log::trace!("Different TypeDef: {ident1:?}.type_def != {ident2:?}.type_def");
      }
    }

    if !compatible {
      // If not compatible, update `seen` cache.
      seen.insert((id1, id2), false);
      log::trace!("Different types: {ident1:?} != {ident2:?}");
    }

    return compatible;
  }

  pub fn is_storage_entry_compatible(&self, seen: &mut HashMap<(u32, u32), bool>, other: &Self, entry: &StorageEntryMetadata<PortableForm>, entry2: &StorageEntryMetadata<PortableForm>) -> bool {
    let mut compatible = true;

    if entry.modifier != entry2.modifier {
      compatible = false;
    }

    // Check storage types.
    match &entry.ty {
      StorageEntryType::Plain(ty1) => {
        match &entry2.ty {
          StorageEntryType::Plain(ty2) => {
            if !self.is_types_compatible(seen, other, ty1.id(), ty2.id()) {
              compatible = false;
            }
          }
          _ => {
            compatible = false;
          }
        }
      }
      StorageEntryType::Map { hashers, key, value } => {
        match &entry2.ty {
          StorageEntryType::Map { hashers: hashers2, key: key2, value: value2 } => {
            if hashers != hashers2 {
              compatible = false;
              log::warn!("Hashers changed on storage entry");
            }
            if !self.is_types_compatible(seen, other, key.id(), key2.id()) {
              compatible = false;
            }
            if !self.is_types_compatible(seen, other, value.id(), value2.id()) {
              compatible = false;
            }
          }
          _ => {
            compatible = false;
          }
        }
      }
    }

    return compatible;
  }

  pub fn is_compatible(&self, other: &Self) -> bool {
    let mut compatible = true;
    let mut seen = HashMap::new();

    // Check each pallet's storage.
    for (pallet_name, storage) in &self.storage {
      match other.storage.get(pallet_name) {
        Some(storage2) => {
          log::trace!("Check pallet: {pallet_name:?}");
          // Check each storage entry in the pallet of both metadata.
          for (name, entry) in &storage.entries {
            match storage2.entries.get(name) {
              Some(entry2) => {
                log::trace!("  -- Storage entry: {name:?}");
                if !self.is_storage_entry_compatible(&mut seen, other, entry, entry2) {
                  compatible = false;
                  log::warn!("Storage entries different: {pallet_name}.{name}");
                }
              }
              None => {
                log::warn!("Removed pallet storage entry: {pallet_name}.{name}");
              }
            }
          }
        }
        None => {
          log::warn!("Removed pallet storage: {pallet_name}");
        }
      }
    }

    return compatible;
  }
}

fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let metadata1 = Metadata::from_file(env::args().nth(1).expect("Missing metadata file1."))?;
  let metadata2 = Metadata::from_file(env::args().nth(2).expect("Missing metadata file2."))?;

  let compatible = metadata1.is_compatible(&metadata2);

  eprintln!("results = {compatible}");

  Ok(())
}

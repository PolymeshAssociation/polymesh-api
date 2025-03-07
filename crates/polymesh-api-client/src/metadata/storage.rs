use super::*;

#[derive(Clone)]
pub struct StorageMetadata {
  pub prefix: String,
  pub entries: BTreeMap<String, StorageEntryMetadata>,
}

impl StorageMetadata {
  #[cfg(feature = "v12")]
  pub fn from_v12_meta(
    md: frame_metadata::v12::StorageMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let prefix = decode_meta(&md.prefix)?.clone();
    let mut entries = BTreeMap::new();

    decode_meta(&md.entries)?
      .iter()
      .try_for_each(|entry| -> Result<()> {
        let entry_md = StorageEntryMetadata::from_v12_meta(entry, lookup)?;
        let name = entry_md.name.clone();
        entries.insert(name, entry_md);
        Ok(())
      })?;

    Ok(Self { prefix, entries })
  }

  #[cfg(feature = "v13")]
  pub fn from_v13_meta(
    md: frame_metadata::v13::StorageMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let prefix = decode_meta(&md.prefix)?.clone();
    let mut entries = BTreeMap::new();

    decode_meta(&md.entries)?
      .iter()
      .try_for_each(|entry| -> Result<()> {
        let entry_md = StorageEntryMetadata::from_v13_meta(entry, lookup)?;
        let name = entry_md.name.clone();
        entries.insert(name, entry_md);
        Ok(())
      })?;

    Ok(Self { prefix, entries })
  }

  #[cfg(feature = "v14")]
  pub fn from_v14_meta(
    md: &frame_metadata::v14::PalletStorageMetadata<PortableForm>,
    types: &PortableRegistry,
  ) -> Result<Self> {
    let prefix = md.prefix.clone();
    let mut entries = BTreeMap::new();

    md.entries.iter().try_for_each(|entry| -> Result<()> {
      let entry_md = StorageEntryMetadata::from_v14_meta(entry, types)?;
      let name = entry_md.name.clone();
      entries.insert(name, entry_md);
      Ok(())
    })?;

    Ok(Self { prefix, entries })
  }
}

#[derive(Clone)]
pub enum StorageHasher {
  Blake2_128,
  Blake2_256,
  Blake2_128Concat,
  Twox128,
  Twox256,
  Twox64Concat,
  Identity,
}

#[cfg(feature = "v12")]
impl From<&frame_metadata::v12::StorageHasher> for StorageHasher {
  fn from(hasher: &frame_metadata::v12::StorageHasher) -> Self {
    use frame_metadata::v12::StorageHasher as MetadataHasher;
    match hasher {
      MetadataHasher::Blake2_128 => Self::Blake2_128,
      MetadataHasher::Blake2_256 => Self::Blake2_256,
      MetadataHasher::Blake2_128Concat => Self::Blake2_128Concat,
      MetadataHasher::Twox128 => Self::Twox128,
      MetadataHasher::Twox256 => Self::Twox256,
      MetadataHasher::Twox64Concat => Self::Twox64Concat,
      MetadataHasher::Identity => Self::Identity,
    }
  }
}

#[cfg(feature = "v13")]
impl From<&frame_metadata::v13::StorageHasher> for StorageHasher {
  fn from(hasher: &frame_metadata::v13::StorageHasher) -> Self {
    use frame_metadata::v13::StorageHasher as MetadataHasher;
    match hasher {
      MetadataHasher::Blake2_128 => Self::Blake2_128,
      MetadataHasher::Blake2_256 => Self::Blake2_256,
      MetadataHasher::Blake2_128Concat => Self::Blake2_128Concat,
      MetadataHasher::Twox128 => Self::Twox128,
      MetadataHasher::Twox256 => Self::Twox256,
      MetadataHasher::Twox64Concat => Self::Twox64Concat,
      MetadataHasher::Identity => Self::Identity,
    }
  }
}

#[cfg(feature = "v14")]
impl From<&frame_metadata::v14::StorageHasher> for StorageHasher {
  fn from(hasher: &frame_metadata::v14::StorageHasher) -> Self {
    use frame_metadata::v14::StorageHasher as MetadataHasher;
    match hasher {
      MetadataHasher::Blake2_128 => Self::Blake2_128,
      MetadataHasher::Blake2_256 => Self::Blake2_256,
      MetadataHasher::Blake2_128Concat => Self::Blake2_128Concat,
      MetadataHasher::Twox128 => Self::Twox128,
      MetadataHasher::Twox256 => Self::Twox256,
      MetadataHasher::Twox64Concat => Self::Twox64Concat,
      MetadataHasher::Identity => Self::Identity,
    }
  }
}

#[derive(Clone)]
pub enum StorageEntryType {
  Plain(TypeId),
  Map {
    hasher: StorageHasher,
    key: TypeId,
    value: TypeId,
    // For double maps or higher
    additional_hashers_keys: Vec<(StorageHasher, TypeId)>,
  },
}

#[derive(Clone)]
pub enum StorageEntryModifier {
  Optional,
  Default,
}

#[cfg(feature = "v12")]
impl From<&frame_metadata::v12::StorageEntryModifier> for StorageEntryModifier {
  fn from(modifier: &frame_metadata::v12::StorageEntryModifier) -> Self {
    use frame_metadata::v12::StorageEntryModifier as MetadataModifier;
    match modifier {
      MetadataModifier::Optional => Self::Optional,
      MetadataModifier::Default => Self::Default,
    }
  }
}

#[cfg(feature = "v13")]
impl From<&frame_metadata::v13::StorageEntryModifier> for StorageEntryModifier {
  fn from(modifier: &frame_metadata::v13::StorageEntryModifier) -> Self {
    use frame_metadata::v13::StorageEntryModifier as MetadataModifier;
    match modifier {
      MetadataModifier::Optional => Self::Optional,
      MetadataModifier::Default => Self::Default,
    }
  }
}

#[cfg(feature = "v14")]
impl From<&frame_metadata::v14::StorageEntryModifier> for StorageEntryModifier {
  fn from(modifier: &frame_metadata::v14::StorageEntryModifier) -> Self {
    use frame_metadata::v14::StorageEntryModifier as MetadataModifier;
    match modifier {
      MetadataModifier::Optional => Self::Optional,
      MetadataModifier::Default => Self::Default,
    }
  }
}

#[derive(Clone)]
pub struct StorageEntryMetadata {
  pub name: String,
  pub modifier: StorageEntryModifier,
  pub ty: StorageEntryType,
  pub default: Vec<u8>,
  pub docs: Docs,
}

impl StorageEntryMetadata {
  #[cfg(feature = "v12")]
  fn from_v12_meta(
    md: &frame_metadata::v12::StorageEntryMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let name = decode_meta(&md.name)?.clone();
    let modifier = (&md.modifier).into();
    let docs = Docs::from_v12_meta(&md.documentation)?;
    let default = decode_meta(&md.default)?.clone();

    let ty = match &md.ty {
      frame_metadata::v12::StorageEntryType::Plain(plain_ty) => {
        let ty_name = decode_meta(plain_ty)?;
        let ty_id = lookup.parse_type(&ty_name)?;
        StorageEntryType::Plain(ty_id)
      }
      frame_metadata::v12::StorageEntryType::Map {
        hasher, key, value, ..
      } => {
        let key_ty = decode_meta(key)?;
        let value_ty = decode_meta(value)?;
        let key_id = lookup.parse_type(&key_ty)?;
        let value_id = lookup.parse_type(&value_ty)?;
        let hasher = hasher.into();

        StorageEntryType::Map {
          hasher,
          key: key_id,
          value: value_id,
          additional_hashers_keys: Vec::new(),
        }
      }
      frame_metadata::v12::StorageEntryType::DoubleMap {
        hasher,
        key1,
        key2,
        value,
        key2_hasher,
      } => {
        let key1_ty = decode_meta(key1)?;
        let key2_ty = decode_meta(key2)?;
        let value_ty = decode_meta(value)?;
        let key1_id = lookup.parse_type(&key1_ty)?;
        let key2_id = lookup.parse_type(&key2_ty)?;
        let value_id = lookup.parse_type(&value_ty)?;
        let hasher1 = (hasher).into();
        let hasher2 = (key2_hasher).into();

        StorageEntryType::Map {
          hasher: hasher1,
          key: key1_id,
          value: value_id,
          additional_hashers_keys: vec![(hasher2, key2_id)],
        }
      }
    };

    Ok(Self {
      name,
      modifier,
      ty,
      default,
      docs,
    })
  }

  #[cfg(feature = "v13")]
  fn from_v13_meta(
    md: &frame_metadata::v13::StorageEntryMetadata,
    lookup: &mut Types,
  ) -> Result<Self> {
    let name = decode_meta(&md.name)?.clone();
    let modifier = (&md.modifier).into();
    let docs = Docs::from_v13_meta(&md.documentation)?;
    let default = decode_meta(&md.default)?.clone();

    let ty = match &md.ty {
      frame_metadata::v13::StorageEntryType::Plain(plain_ty) => {
        let ty_name = decode_meta(plain_ty)?;
        let ty_id = lookup.parse_type(&ty_name)?;
        StorageEntryType::Plain(ty_id)
      }
      frame_metadata::v13::StorageEntryType::Map {
        hasher, key, value, ..
      } => {
        let key_ty = decode_meta(key)?;
        let value_ty = decode_meta(value)?;
        let key_id = lookup.parse_type(&key_ty)?;
        let value_id = lookup.parse_type(&value_ty)?;
        let hasher = (hasher).into();

        StorageEntryType::Map {
          hasher,
          key: key_id,
          value: value_id,
          additional_hashers_keys: Vec::new(),
        }
      }
      frame_metadata::v13::StorageEntryType::DoubleMap {
        hasher,
        key1,
        key2,
        value,
        key2_hasher,
      } => {
        let key1_ty = decode_meta(key1)?;
        let key2_ty = decode_meta(key2)?;
        let value_ty = decode_meta(value)?;
        let key1_id = lookup.parse_type(&key1_ty)?;
        let key2_id = lookup.parse_type(&key2_ty)?;
        let value_id = lookup.parse_type(&value_ty)?;
        let hasher1 = (hasher).into();
        let hasher2 = (key2_hasher).into();

        StorageEntryType::Map {
          hasher: hasher1,
          key: key1_id,
          value: value_id,
          additional_hashers_keys: vec![(hasher2, key2_id)],
        }
      }
      frame_metadata::v13::StorageEntryType::NMap {
        hashers,
        keys,
        value,
      } => {
        let keys_ty = decode_meta(keys)?;
        let value_ty = decode_meta(value)?;
        let hashers_vec = decode_meta(hashers)?;

        // Process first key and hasher
        let first_key_ty = &keys_ty[0];
        let first_key_id = lookup.parse_type(first_key_ty)?;
        let first_hasher = (&hashers_vec[0]).into();

        // Process additional keys and hashers
        let mut additional_hashers_keys = Vec::new();
        for i in 1..keys_ty.len() {
          let key_ty = &keys_ty[i];
          let key_id = lookup.parse_type(key_ty)?;
          let hasher = (&hashers_vec[i]).into();
          additional_hashers_keys.push((hasher, key_id));
        }

        let value_id = lookup.parse_type(&value_ty)?;

        StorageEntryType::Map {
          hasher: first_hasher,
          key: first_key_id,
          value: value_id,
          additional_hashers_keys,
        }
      }
    };

    Ok(Self {
      name,
      modifier,
      ty,
      default,
      docs,
    })
  }

  #[cfg(feature = "v14")]
  fn from_v14_meta(
    md: &frame_metadata::v14::StorageEntryMetadata<PortableForm>,
    types: &PortableRegistry,
  ) -> Result<Self> {
    let name = md.name.clone();
    let modifier = (&md.modifier).into();
    let docs = Docs::from_v14_meta(&md.docs);
    let default = md.default.clone();

    let ty = match &md.ty {
      frame_metadata::v14::StorageEntryType::Plain(plain_ty) => {
        StorageEntryType::Plain(TypeId::from(plain_ty.id))
      }
      frame_metadata::v14::StorageEntryType::Map {
        hashers,
        key,
        value,
      } => {
        if hashers.len() == 1 {
          // Simple map
          let hasher = (&hashers[0]).into();
          StorageEntryType::Map {
            hasher,
            key: TypeId::from(key.id),
            value: TypeId::from(value.id),
            additional_hashers_keys: Vec::new(),
          }
        } else if hashers.len() > 1 {
          // NMap (double map or higher)
          let first_hasher = (&hashers[0]).into();

          // For NMap, we need to extract the individual key types from the tuple
          let key_type = types
            .resolve(key.id)
            .ok_or_else(|| Error::MetadataParseFailed("Failed to resolve NMap key type".into()))?;

          let mut additional_hashers_keys = Vec::new();

          if let TypeDef::Tuple(tuple) = key_type.type_def() {
            if tuple.fields.len() != hashers.len() {
              return Err(Error::MetadataParseFailed(
                "Mismatch between hashers and key types count".into(),
              ));
            }

            // Skip the first key as it's handled separately
            for i in 1..hashers.len() {
              let key_id = tuple.fields[i].id();
              let hasher = (&hashers[i]).into();
              additional_hashers_keys.push((hasher, TypeId::from(key_id)));
            }

            StorageEntryType::Map {
              hasher: first_hasher,
              key: TypeId::from(tuple.fields[0].id()),
              value: TypeId::from(value.id),
              additional_hashers_keys,
            }
          } else {
            return Err(Error::MetadataParseFailed(
              "Expected tuple type for NMap keys".into(),
            ));
          }
        } else {
          return Err(Error::MetadataParseFailed(
            "Empty hashers list in map storage entry".into(),
          ));
        }
      }
    };

    Ok(Self {
      name,
      modifier,
      ty,
      default,
      docs,
    })
  }
}

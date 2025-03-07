use super::*;
use sp_core::hashing::{blake2_128, blake2_256, twox_128, twox_256, twox_64};

/// Metadata for a pallet's storage.
///
/// Contains information about the storage prefix and all storage entries in this pallet.
#[derive(Clone)]
pub struct StorageMetadata {
  /// The prefix used for all storage items in this pallet.
  pub prefix: String,
  /// The storage entries in this pallet, keyed by entry name.
  pub entries: BTreeMap<String, StorageEntryMetadata>,
}

impl StorageMetadata {
  /// Creates storage metadata from V12 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V12 storage metadata
  /// * `lookup` - Types registry for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage metadata, or an error if parsing fails.
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

  /// Creates storage metadata from V13 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V13 storage metadata
  /// * `lookup` - Types registry for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage metadata, or an error if parsing fails.
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

  /// Creates storage metadata from V14 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V14 pallet storage metadata
  /// * `types` - Registry of portable types for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage metadata, or an error if parsing fails.
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

  /// Computes the pallet prefix hash, which is the xxhash128 of the pallet's storage prefix.
  ///
  /// # Returns
  ///
  /// The xxhash128 of the pallet prefix as a vector of bytes.
  pub fn pallet_prefix_hash(&self) -> Vec<u8> {
    twox_128(self.prefix.as_bytes()).to_vec()
  }

  /// Computes the storage prefix hash for a given entry, which is the pallet prefix hash
  /// followed by the xxhash128 of the entry name.
  ///
  /// # Arguments
  ///
  /// * `entry_name` - The name of the storage entry
  ///
  /// # Returns
  ///
  /// The complete storage prefix hash as a vector of bytes.
  pub fn storage_prefix_hash(&self, entry_name: &str) -> Result<Vec<u8>> {
    let entry = self.entries.get(entry_name).ok_or_else(|| {
      Error::StorageKeyGenerationFailed(format!("Storage entry '{}' not found", entry_name))
    })?;
    Ok(entry.entry_prefix_hash(&self.pallet_prefix_hash()))
  }

  /// Computes the full storage key for a given entry, given its keys (if any).
  /// This is a convenience method that delegates to the corresponding `StorageEntryMetadata`.
  ///
  /// # Arguments
  ///
  /// * `entry_name` - The name of the storage entry
  /// * `keys` - The keys for this storage entry, if it's a map
  ///
  /// # Returns
  ///
  /// The complete storage key as a vector of bytes, or an error if the entry doesn't exist or
  /// the provided keys don't match the storage entry type.
  pub fn storage_key(&self, entry_name: &str, keys: &[Vec<u8>]) -> Result<Vec<u8>> {
    let entry = self.entries.get(entry_name).ok_or_else(|| {
      Error::StorageKeyGenerationFailed(format!("Storage entry '{}' not found", entry_name))
    })?;
    entry.storage_key(&self.pallet_prefix_hash(), keys)
  }
}

/// The hashing algorithm used for generating storage keys.
#[derive(Clone)]
pub enum StorageHasher {
  /// Blake2 128-bit hash.
  Blake2_128,
  /// Blake2 256-bit hash.
  Blake2_256,
  /// Blake2 128-bit hash followed by the input data.
  Blake2_128Concat,
  /// XX 128-bit hash.
  Twox128,
  /// XX 256-bit hash.
  Twox256,
  /// XX 64-bit hash followed by the input data.
  Twox64Concat,
  /// Identity hashing (no hashing, data used as-is).
  Identity,
}

impl StorageHasher {
  /// Apply this hasher to the given data.
  ///
  /// # Arguments
  ///
  /// * `data` - The input data to hash
  ///
  /// # Returns
  ///
  /// The hashed data as a vector of bytes.
  pub fn hash_data(&self, data: &[u8]) -> Vec<u8> {
    match self {
      Self::Blake2_128 => blake2_128(data).to_vec(),
      Self::Blake2_256 => blake2_256(data).to_vec(),
      Self::Blake2_128Concat => {
        let mut result = blake2_128(data).to_vec();
        result.extend_from_slice(data);
        result
      }
      Self::Twox128 => twox_128(data).to_vec(),
      Self::Twox256 => twox_256(data).to_vec(),
      Self::Twox64Concat => {
        let mut result = twox_64(data).to_vec();
        result.extend_from_slice(data);
        result
      }
      Self::Identity => data.to_vec(),
    }
  }
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

/// Type information for a storage entry.
///
/// Represents either a plain storage entry (single value) or a map with one or more keys.
#[derive(Clone)]
pub enum StorageEntryType {
  /// A simple storage entry with a single value of the given type.
  Plain(TypeId),
  /// A storage map from keys to values.
  Map {
    /// The hashing algorithm used for the first key.
    hasher: StorageHasher,
    /// The type ID of the first key.
    key: TypeId,
    /// The type ID of the value.
    value: TypeId,
    /// For NMaps (double maps or higher), contains pairs of (hasher, key_type)
    /// for additional keys beyond the first one.
    additional_hashers_keys: Vec<(StorageHasher, TypeId)>,
  },
}

/// Modifier for a storage entry that indicates how the entry behaves when not set.
#[derive(Clone)]
pub enum StorageEntryModifier {
  /// If the entry doesn't exist, it's reported as `None`.
  Optional,
  /// If the entry doesn't exist, the default value is returned.
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

/// Metadata for a single storage entry within a pallet.
#[derive(Clone)]
pub struct StorageEntryMetadata {
  /// The name of the storage entry.
  pub name: String,
  /// The modifier indicating behavior when the entry doesn't exist.
  pub modifier: StorageEntryModifier,
  /// The type information for this storage entry.
  pub ty: StorageEntryType,
  /// The default value for this entry as SCALE-encoded bytes.
  pub default: Vec<u8>,
  /// Documentation for this storage entry.
  pub docs: Docs,
}

impl StorageEntryMetadata {
  /// Creates storage entry metadata from V12 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V12 storage entry metadata
  /// * `lookup` - Types registry for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage entry metadata, or an error if parsing fails.
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

  /// Creates storage entry metadata from V13 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V13 storage entry metadata
  /// * `lookup` - Types registry for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage entry metadata, or an error if parsing fails.
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

  /// Creates storage entry metadata from V14 metadata format.
  ///
  /// # Arguments
  ///
  /// * `md` - The V14 storage entry metadata
  /// * `types` - Registry of portable types for resolving type references
  ///
  /// # Returns
  ///
  /// The parsed storage entry metadata, or an error if parsing fails.
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

  /// Computes the entry prefix hash, which is the pallet prefix hash ++ xxhash128 of the entry name.
  ///
  /// # Arguments
  ///
  /// * `pallet_prefix_hash` - The hash of the pallet prefix
  ///
  /// # Returns
  ///
  /// The complete entry prefix hash as a vector of bytes.
  pub fn entry_prefix_hash(&self, pallet_prefix_hash: &[u8]) -> Vec<u8> {
    let mut result = pallet_prefix_hash.to_vec();
    result.extend_from_slice(&twox_128(self.name.as_bytes()));
    result
  }

  /// Computes the full storage key for this entry, given its keys (if any).
  ///
  /// # Arguments
  ///
  /// * `pallet_prefix_hash` - The hash of the pallet prefix
  /// * `keys` - The keys for this storage entry, if it's a map
  ///
  /// # Returns
  ///
  /// The complete storage key as a vector of bytes, or an error if provided keys
  /// don't match the storage entry type.
  pub fn storage_key(&self, pallet_prefix_hash: &[u8], keys: &[Vec<u8>]) -> Result<Vec<u8>> {
    // Start with the entry prefix hash (pallet_prefix_hash + entry_name_hash)
    let mut key = self.entry_prefix_hash(pallet_prefix_hash);

    match &self.ty {
      StorageEntryType::Plain(_) => {
        // For plain storage, no additional keys are needed
        if !keys.is_empty() {
          return Err(Error::StorageKeyGenerationFailed(
            "Plain storage takes no keys".into(),
          ));
        }
      }
      StorageEntryType::Map {
        hasher,
        additional_hashers_keys,
        ..
      } => {
        // For maps, we need exactly 1 + additional_hashers_keys.len() keys
        let expected_keys = 1 + additional_hashers_keys.len();
        if keys.len() != expected_keys {
          return Err(Error::StorageKeyGenerationFailed(format!(
            "Expected {} keys for this map, got {}",
            expected_keys,
            keys.len()
          )));
        }

        // Hash first key with its hasher
        key.extend_from_slice(&hasher.hash_data(&keys[0]));

        // Hash additional keys with their respective hashers
        for (i, (hasher, _)) in additional_hashers_keys.iter().enumerate() {
          key.extend_from_slice(&hasher.hash_data(&keys[i + 1]));
        }
      }
    }

    Ok(key)
  }
}

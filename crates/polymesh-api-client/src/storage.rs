use core::marker::PhantomData;

use async_stream::try_stream;
use futures_core::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::format;
use codec::Decode;
use sp_std::prelude::*;

use crate::*;

/// Query chain map/double map storage with a common prefix.
///
/// The `state_getKeysPaged` API is used to get batches of keys.
pub struct StoragePaged<K, V> {
  client: Client,
  prefix: StorageKey,
  key_hash_len: Option<usize>,
  at: Option<BlockHash>,
  batch_size: usize,
  start_key: Option<StorageKey>,
  finished: bool,
  _phantom: PhantomData<(K, V)>,
}

impl<K: Decode, V: Decode> StoragePaged<K, V> {
  pub fn new(
    client: &Client,
    prefix: StorageKey,
    key_hash_len: Option<usize>,
    at: Option<BlockHash>,
  ) -> Self {
    Self {
      client: client.clone(),
      prefix,
      key_hash_len,
      at,
      batch_size: 10,
      start_key: None,
      finished: false,
      _phantom: PhantomData::default(),
    }
  }

  /// Change the `batch_size` (default is 10).
  pub fn batch_size(mut self, batch_size: usize) -> Self {
    self.batch_size = batch_size;
    self
  }

  fn get_hashed_key<'a>(&self, key: &'a StorageKey) -> Result<&'a [u8]> {
    let h_len = match self.key_hash_len {
      Some(l) => l,
      None => {
        return Err(Error::DecodeTypeFailed(format!(
          "Failed to decode storage key: hasher isn't reversible"
        )));
      }
    };
    let p_len = self.prefix.0.len();
    if key.0.len() < (p_len + h_len) {
      return Err(Error::DecodeTypeFailed(format!(
        "Failed to decode storage key: too short"
      )));
    }
    let (key_prefix, key) = key.0.split_at(p_len);
    if key_prefix != self.prefix.0.as_slice() {
      return Err(Error::DecodeTypeFailed(format!(
        "Invalid storage key, the prefix doesn't match"
      )));
    }
    Ok(&key[h_len..])
  }

  async fn next_page(&mut self) -> Result<Option<Vec<StorageKey>>> {
    if self.finished {
      return Ok(None);
    }
    let keys = self
      .client
      .get_storage_keys_paged(
        &self.prefix,
        self.batch_size,
        self.start_key.as_ref(),
        self.at,
      )
      .await?;
    if keys.len() < self.batch_size {
      self.finished = true;
    } else {
      self.start_key = keys.last().cloned();
    }
    Ok(Some(keys))
  }

  /// Async stream to get key/value pairs.
  pub fn entries(mut self) -> impl Stream<Item = Result<(K, Option<V>)>> {
    try_stream! {
      while let Some(keys) = self.next_page().await? {
        for storage_key in keys {
          // Decode key.
          let mut data = self.get_hashed_key(&storage_key)?;
          let key = K::decode(&mut data)?;
          // Get value from chain storage.
          let value = self.client.get_storage_by_key(storage_key, self.at).await?;
          yield (key, value);
        }
      }
    }
  }

  /// Async stream to get only keys.
  pub fn keys(mut self) -> impl Stream<Item = Result<K>> {
    try_stream! {
      while let Some(keys) = self.next_page().await? {
        for key in keys {
          let mut data = self.get_hashed_key(&key)?;
          yield K::decode(&mut data)?;
        }
      }
    }
  }

  /// Async stream to get only values.
  pub fn values(mut self) -> impl Stream<Item = Result<Option<V>>> {
    try_stream! {
      while let Some(keys) = self.next_page().await? {
        for key in keys {
          let value = self.client.get_storage_by_key(key, self.at).await?;
          yield value;
        }
      }
    }
  }
}

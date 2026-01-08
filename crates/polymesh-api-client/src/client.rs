#[cfg(not(feature = "std"))]
use alloc::{collections::btree_map::BTreeMap, sync::Arc};
#[cfg(feature = "std")]
use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee::core::client::BatchResponse;
pub use jsonrpsee::core::client::Subscription;
use jsonrpsee::core::params::{ArrayParams, BatchRequestBuilder};
use jsonrpsee::rpc_params;

use codec::Decode;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_json::Value;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use sp_std::prelude::*;

#[cfg(feature = "type_info")]
use frame_metadata::RuntimeMetadataPrefixed;

use crate::rpc::*;
use crate::*;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RuntimeVersion {
  pub spec_name: String,
  pub impl_name: String,
  pub authoring_version: u32,
  pub spec_version: u32,
  pub impl_version: u32,
  #[cfg_attr(feature = "serde", serde(default))]
  pub transaction_version: u32,

  #[cfg_attr(feature = "serde", serde(flatten))]
  #[cfg(feature = "serde")]
  pub extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SystemProperties {
  pub ss58_format: u16,
  pub token_decimals: u32,
  pub token_symbol: String,
}

struct InnerClient {
  rpc: RpcClient,
  runtime_version: RuntimeVersion,
  #[cfg(feature = "type_info")]
  metadata: RuntimeMetadataPrefixed,
  genesis_hash: BlockHash,
}

impl InnerClient {
  async fn new(url: &str) -> Result<Self> {
    let rpc = RpcClient::new(url).await?;
    let runtime_version = Self::rpc_get_runtime_version(&rpc, None)
      .await?
      .ok_or_else(|| Error::RpcClient(format!("Failed to get RuntimeVersion")))?;
    #[cfg(feature = "type_info")]
    let metadata = Self::rpc_get_metadata(&rpc, None)
      .await?
      .ok_or_else(|| Error::RpcClient(format!("Failed to get chain metadata")))?;
    let genesis_hash = Self::rpc_get_block_hash(&rpc, 0)
      .await?
      .ok_or_else(|| Error::RpcClient(format!("Failed to get chain Genesis hash")))?;
    Ok(Self {
      rpc,
      runtime_version,
      #[cfg(feature = "type_info")]
      metadata,
      genesis_hash,
    })
  }

  fn get_transaction_version(&self) -> i64 {
    self.runtime_version.transaction_version as i64
  }

  #[cfg(feature = "type_info")]
  fn get_metadata(&self) -> &RuntimeMetadataPrefixed {
    &self.metadata
  }

  fn get_genesis_hash(&self) -> BlockHash {
    self.genesis_hash
  }

  async fn get_additional_signed(&self, lifetime: Option<u64>) -> Result<(AdditionalSigned, Era)> {
    let mut addititional = AdditionalSigned {
      spec_version: self.runtime_version.spec_version,
      tx_version: self.runtime_version.transaction_version,
      genesis_hash: self.genesis_hash,
      current_hash: self.genesis_hash,
      metadata_hash: None,
    };
    let era = match lifetime {
      Some(0) => Era::immortal(),
      lifetime => {
        let current = self
          .get_block_header(None)
          .await?
          .ok_or_else(|| Error::RpcClient("Failed to get current block".into()))?;
        let number = current.number;
        // Need to use the current block hash.
        addititional.current_hash = current.hash();
        Era::mortal(number, lifetime)
      }
    };

    Ok((addititional, era))
  }

  #[cfg(feature = "serde")]
  async fn request<'a, R>(&self, method: &'a str, params: ArrayParams) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self.rpc.request(method, params).await
  }

  #[cfg(feature = "serde")]
  async fn batch_request<'a, R>(
    &self,
    batch: BatchRequestBuilder<'a>,
  ) -> Result<BatchResponse<'a, R>>
  where
    R: DeserializeOwned + Default + Clone + alloc::fmt::Debug + 'a,
  {
    self.rpc.batch_request(batch).await
  }

  #[cfg(feature = "serde")]
  async fn subscribe<'a, Notif>(
    &self,
    subscribe_method: &'a str,
    params: ArrayParams,
    unsubscribe_method: &'a str,
  ) -> Result<Subscription<Notif>>
  where
    Notif: DeserializeOwned,
  {
    self
      .rpc
      .subscribe(subscribe_method, params, unsubscribe_method)
      .await
  }

  async fn rpc_get_block_hash(rpc: &RpcClient, block_number: u32) -> Result<Option<BlockHash>> {
    let params = rpc_params!(block_number);
    Ok(rpc.request("chain_getBlockHash", params).await?)
  }

  /// Get the header of a block.
  async fn get_block_header(&self, block: Option<BlockHash>) -> Result<Option<Header>> {
    Ok(self.request("chain_getHeader", rpc_params!(block)).await?)
  }

  /// Get the block hash for a `block_number`.
  async fn get_block_hash(&self, block_number: u32) -> Result<Option<BlockHash>> {
    Self::rpc_get_block_hash(&self.rpc, block_number).await
  }

  async fn rpc_get_runtime_version(
    rpc: &RpcClient,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeVersion>> {
    let params = rpc_params!(block);
    rpc.request("state_getRuntimeVersion", params).await
  }

  /// Get the RuntimeVersion of a block.
  async fn get_block_runtime_version(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeVersion>> {
    Self::rpc_get_runtime_version(&self.rpc, block).await
  }

  #[cfg(feature = "type_info")]
  async fn rpc_get_metadata(
    rpc: &RpcClient,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeMetadataPrefixed>> {
    use hex::FromHex;
    let params = rpc_params!(block);
    let hex: String = rpc.request("state_getMetadata", params).await?;

    let bytes = Vec::from_hex(&hex[2..])?;
    Ok(Some(RuntimeMetadataPrefixed::decode(
      &mut bytes.as_slice(),
    )?))
  }

  /// Get the RuntimeMetadata of a block.
  #[cfg(feature = "type_info")]
  async fn get_block_metadata(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeMetadataPrefixed>> {
    Self::rpc_get_metadata(&self.rpc, block).await
  }
}

#[derive(Clone)]
pub struct Client {
  inner: Arc<InnerClient>,
}

impl Client {
  pub async fn new(url: &str) -> Result<Self> {
    Ok(Self {
      inner: Arc::new(InnerClient::new(url).await?),
    })
  }

  pub fn get_transaction_version(&self) -> i64 {
    self.inner.get_transaction_version()
  }

  #[cfg(feature = "type_info")]
  pub fn get_metadata(&self) -> &RuntimeMetadataPrefixed {
    self.inner.get_metadata()
  }

  pub fn get_genesis_hash(&self) -> BlockHash {
    self.inner.get_genesis_hash()
  }

  pub async fn get_additional_signed(
    &self,
    lifetime: Option<u64>,
  ) -> Result<(AdditionalSigned, Era)> {
    self.inner.get_additional_signed(lifetime).await
  }

  /// Get the `SystemProperties` of the chain.
  pub async fn get_system_properties(&self) -> Result<SystemProperties> {
    self.request("system_properties", rpc_params!()).await
  }

  pub async fn get_storage_keys_paged(
    &self,
    prefix: &StorageKey,
    count: usize,
    start_key: Option<&StorageKey>,
    at: Option<BlockHash>,
  ) -> Result<Vec<StorageKey>> {
    let params = rpc_params!(prefix, count, start_key.unwrap_or(prefix), at);
    self
      .request::<Vec<StorageKey>>("state_getKeysPaged", params)
      .await
  }

  pub async fn get_storage_by_key<T: Decode>(
    &self,
    key: StorageKey,
    at: Option<BlockHash>,
  ) -> Result<Option<T>> {
    let value = self
      .get_storage_data_by_key(key, at)
      .await?
      .map(|data| T::decode(&mut data.0.as_slice()))
      .transpose()?;
    Ok(value)
  }

  pub async fn get_storage_data_by_key(
    &self,
    key: StorageKey,
    at: Option<BlockHash>,
  ) -> Result<Option<StorageData>> {
    Ok(
      self
        .request("state_getStorage", rpc_params!(key, at))
        .await?,
    )
  }

  /// Subscribe to new blocks.
  pub async fn subscribe_blocks(&self) -> Result<Subscription<Header>> {
    Ok(
      self
        .subscribe(
          "chain_subscribeNewHeads",
          rpc_params!(),
          "chain_unsubscribeNewHeads",
        )
        .await?,
    )
  }

  /// Subscribe to new finalized blocks.
  pub async fn subscribe_finalized_blocks(&self) -> Result<Subscription<Header>> {
    Ok(
      self
        .subscribe(
          "chain_subscribeFinalizedHeads",
          rpc_params!(),
          "chain_unsubscribeFinalizedHeads",
        )
        .await?,
    )
  }

  /// Submit and watch a transaction.
  pub async fn submit_and_watch(&self, tx_hex: String) -> Result<Subscription<TransactionStatus>> {
    Ok(
      self
        .subscribe(
          "author_submitAndWatchExtrinsic",
          rpc_params!(tx_hex),
          "author_unwatchExtrinsic",
        )
        .await?,
    )
  }

  /// Make a RPC request to the node.
  #[cfg(feature = "serde")]
  pub async fn request<'a, R>(&self, method: &'a str, params: ArrayParams) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self.inner.request(method, params).await
  }

  /// Make a batch of RPC requests to the node.
  #[cfg(feature = "serde")]
  pub async fn batch_request<'a, R>(
    &self,
    batch: BatchRequestBuilder<'a>,
  ) -> Result<BatchResponse<'a, R>>
  where
    R: DeserializeOwned + Default + Clone + alloc::fmt::Debug + 'a,
  {
    self.inner.batch_request(batch).await
  }

  /// Subscribe to RPC updates.
  #[cfg(feature = "serde")]
  pub async fn subscribe<'a, Notif>(
    &self,
    subscribe_method: &'a str,
    params: ArrayParams,
    unsubscribe_method: &'a str,
  ) -> Result<Subscription<Notif>>
  where
    Notif: DeserializeOwned,
  {
    self
      .inner
      .subscribe(subscribe_method, params, unsubscribe_method)
      .await
  }

  /// Get the current finalized block hash.
  pub async fn get_finalized_block(&self) -> Result<BlockHash> {
    Ok(
      self
        .request("chain_getFinalizedHead", rpc_params!())
        .await?,
    )
  }

  /// Get a block.
  pub async fn get_signed_block(&self, block: Option<BlockHash>) -> Result<Option<SignedBlock>> {
    Ok(self.request("chain_getBlock", rpc_params!(block)).await?)
  }

  /// Get a block.
  pub async fn get_block(&self, block: Option<BlockHash>) -> Result<Option<Block>> {
    let block = self.get_signed_block(block).await?;
    Ok(block.map(|b| b.block))
  }

  /// Get find extrinsic index in block.
  /// The extrinsic index is used to filter the block events for that extrinsic.
  pub async fn find_extrinsic_block_index(
    &self,
    block: BlockHash,
    tx_hash: TxHash,
  ) -> Result<Option<usize>> {
    let block = self.get_block(Some(block)).await?;
    Ok(block.and_then(|b| b.find_extrinsic(tx_hash)))
  }

  /// Get the header of a block.
  pub async fn get_block_header(&self, block: Option<BlockHash>) -> Result<Option<Header>> {
    self.inner.get_block_header(block).await
  }

  /// Get the block hash for a `block_number`.
  pub async fn get_block_hash(&self, block_number: u32) -> Result<Option<BlockHash>> {
    self.inner.get_block_hash(block_number).await
  }

  /// Subscribe to RuntimeVersion updates.
  pub async fn subscribe_runtime_version(&self) -> Result<Subscription<RuntimeVersion>> {
    Ok(
      self
        .subscribe(
          "chain_subscribeRuntimeVersion",
          rpc_params!(),
          "chain_unsubscribeRuntimeVersion",
        )
        .await?,
    )
  }

  /// Get the RuntimeVersion of a block.
  pub async fn get_block_runtime_version(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeVersion>> {
    self.inner.get_block_runtime_version(block).await
  }

  /// Get the RuntimeMetadata of a block.
  #[cfg(feature = "type_info")]
  pub async fn get_block_metadata(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Option<RuntimeMetadataPrefixed>> {
    self.inner.get_block_metadata(block).await
  }
}

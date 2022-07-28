use std::collections::HashMap;

use jsonrpsee::core::client::Subscription;
use jsonrpsee::rpc_params;
use jsonrpsee::types::ParamsSer;

use codec::Decode;

use sp_core::storage::{StorageData, StorageKey};

use hex::FromHex;

use serde_json::Value;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use frame_metadata::RuntimeMetadataPrefixed;

use crate::rpc::*;
use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
  pub spec_name: String,
  pub impl_name: String,
  pub authoring_version: u32,
  pub spec_version: u32,
  pub impl_version: u32,
  #[serde(default)]
  pub transaction_version: u32,

  #[serde(flatten)]
  pub extra: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemProperties {
  pub ss58_format: u16,
  pub token_decimals: u32,
  pub token_symbol: String,
}

#[derive(Debug)]
pub struct Client {
  rpc: RpcClient,
  runtime_version: RuntimeVersion,
  metadata: RuntimeMetadataPrefixed,
  genesis_hash: BlockHash,
}

impl Client {
  pub async fn new(url: &str) -> Result<Self> {
    let rpc = RpcClient::new(url).await?;
    let runtime_version = Self::rpc_get_runtime_version(&rpc, None).await?;
    let metadata = Self::rpc_get_metadata(&rpc, None).await?;
    let genesis_hash = Self::rpc_get_block_hash(&rpc, 0).await?;
    Ok(Self {
      rpc,
      runtime_version,
      metadata,
      genesis_hash,
    })
  }

  pub fn get_transaction_version(&self) -> i64 {
    self.runtime_version.transaction_version as i64
  }

  pub fn get_metadata(&self) -> &RuntimeMetadataPrefixed {
    &self.metadata
  }

  pub fn get_signed_extra(&self) -> AdditionalSigned {
    (
      self.runtime_version.spec_version,
      self.runtime_version.transaction_version,
      self.genesis_hash,
      self.genesis_hash,
      (),
      (),
      (),
    )
  }

  /// Get the `SystemProperties` of the chain.
  pub async fn get_system_properties(&self) -> Result<SystemProperties> {
    Ok(self.request("system_properties", rpc_params!()).await?)
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
        .rpc
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
        .rpc
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
        .rpc
        .subscribe(
          "author_submitAndWatchExtrinsic",
          rpc_params!(tx_hex),
          "author_unwatchExtrinsic",
        )
        .await?,
    )
  }

  /// Make a RPC request to the node.
  pub async fn request<'a, R>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<R>
  where
    R: DeserializeOwned,
  {
    Ok(self.rpc.request(method, params).await?)
  }

  /// Make a batch of RPC requests to the node.
  pub async fn batch_request<'a, R>(
    &self,
    batch: Vec<(&'a str, Option<ParamsSer<'a>>)>,
  ) -> Result<Vec<R>>
  where
    R: DeserializeOwned + Default + Clone,
  {
    Ok(self.rpc.batch_request(batch).await?)
  }

  /// Get the current finalized block hash.
  pub async fn get_finalized_block(&self) -> Result<BlockHash> {
    Ok(
      self
        .rpc
        .request("chain_getFinalizedHead", rpc_params!())
        .await?,
    )
  }

  /// Get a block.
  pub async fn get_signed_block(&self, block: Option<BlockHash>) -> Result<Option<SignedBlock>> {
    Ok(
      self
        .rpc
        .request("chain_getBlock", rpc_params!(block))
        .await?,
    )
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
    Ok(
      self
        .rpc
        .request("chain_getHeader", rpc_params!(block))
        .await?,
    )
  }

  async fn rpc_get_block_hash(rpc: &RpcClient, block_number: u32) -> Result<BlockHash> {
    let params = rpc_params!(block_number);
    Ok(rpc.request("chain_getBlockHash", params).await?)
  }

  /// Get the block hash for a `block_number`.
  pub async fn get_block_hash(&self, block_number: u32) -> Result<BlockHash> {
    Ok(Self::rpc_get_block_hash(&self.rpc, block_number).await?)
  }

  /// Subscribe to RuntimeVersion updates.
  pub async fn subscribe_runtime_version(&self) -> Result<Subscription<RuntimeVersion>> {
    Ok(
      self
        .rpc
        .subscribe(
          "chain_subscribeRuntimeVersion",
          rpc_params!(),
          "chain_unsubscribeRuntimeVersion",
        )
        .await?,
    )
  }

  async fn rpc_get_runtime_version(
    rpc: &RpcClient,
    block: Option<BlockHash>,
  ) -> Result<RuntimeVersion> {
    let params = rpc_params!(block);
    Ok(rpc.request("state_getRuntimeVersion", params).await?)
  }

  /// Get the RuntimeVersion of a block.
  pub async fn get_block_runtime_version(
    &self,
    block: Option<BlockHash>,
  ) -> Result<RuntimeVersion> {
    Ok(Self::rpc_get_runtime_version(&self.rpc, block).await?)
  }

  async fn rpc_get_metadata(
    rpc: &RpcClient,
    block: Option<BlockHash>,
  ) -> Result<RuntimeMetadataPrefixed> {
    let params = rpc_params!(block);
    let hex: String = rpc.request("state_getMetadata", params).await?;

    let bytes = Vec::from_hex(&hex[2..])?;
    Ok(RuntimeMetadataPrefixed::decode(&mut bytes.as_slice())?)
  }

  /// Get the RuntimeMetadata of a block.
  pub async fn get_block_metadata(
    &self,
    block: Option<BlockHash>,
  ) -> Result<RuntimeMetadataPrefixed> {
    Ok(Self::rpc_get_metadata(&self.rpc, block).await?)
  }
}

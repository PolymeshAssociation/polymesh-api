use jsonrpsee::core::client::Subscription;
use jsonrpsee::rpc_params;
use jsonrpsee::types::ParamsSer;

use codec::{Decode, Encode};

use sp_core::{
  sr25519,
  storage::{StorageData, StorageKey},
  Pair,
};
use sp_runtime::generic::Era;

use hex::FromHex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use frame_metadata::RuntimeMetadataPrefixed;
use sp_version::RuntimeVersion;

use async_trait::async_trait;

use crate::block::*;
use crate::error::*;
use crate::rpc::*;

#[async_trait]
pub trait ChainApi {
  type RuntimeCall: Clone + Encode + std::fmt::Debug;
  type RuntimeEvent: Clone + Decode + std::fmt::Debug;

  async fn get_nonce(&self, account: AccountId) -> Result<u32>;

  async fn block_events(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Vec<EventRecord<Self::RuntimeEvent>>>;

  fn client(&self) -> &Client;
}

pub struct SimpleSigner {
  pub pair: sr25519::Pair,
  pub nonce: u32,
  pub account: AccountId,
}

impl SimpleSigner {
  pub fn new(pair: sr25519::Pair) -> Self {
    let account = AccountId::new(pair.public().into());
    Self {
      pair,
      nonce: 0,
      account,
    }
  }

  pub async fn submit_and_watch<'api, Api: ChainApi>(
    &mut self,
    call: &Call<'api, Api>,
  ) -> Result<CallResults<'api, Api>> {
    let client = call.api.client();
    // Query account nonce.
    if self.nonce == 0 {
      self.nonce = call.api.get_nonce(self.account.clone()).await?;
    }

    let encoded_call = call.encoded();
    let extra = Extra::new(Era::Immortal, self.nonce);
    let payload = SignedPayload::new(&encoded_call, &extra, client.get_signed_extra());

    let sig = payload.using_encoded(|p| self.pair.sign(p));

    let xt = ExtrinsicV4::signed(self.account.clone(), sig.into(), extra, encoded_call);

    let res = call.submit_and_watch(xt).await?;

    // Update nonce if the call was submitted.
    self.nonce += 1;

    Ok(res)
  }
}

pub struct CallResults<'api, Api: ChainApi> {
  api: &'api Api,
  sub: Option<Subscription<TransactionStatus>>,
  tx_hash: TxHash,
  status: Option<TransactionStatus>,
  block: Option<BlockHash>,
  events: Option<EventRecords<Api::RuntimeEvent>>,
  finalized: bool,
}

impl<'api, Api: ChainApi> CallResults<'api, Api> {
  pub fn new(api: &'api Api, sub: Subscription<TransactionStatus>, tx_hash: TxHash) -> Self {
    Self {
      api,
      sub: Some(sub),
      tx_hash,
      status: None,
      block: None,
      events: None,
      finalized: false,
    }
  }

  async fn next_status(&mut self) -> Result<bool> {
    if let Some(sub) = &mut self.sub {
      match sub.next().await {
        None => {
          // End of stream, no more updates possible.
          self.sub = None;
          Ok(false)
        }
        Some(Ok(status)) => {
          use TransactionStatus::*;
          // Got an update.
          match status {
            InBlock(block) => {
              self.block = Some(block);
            }
            Finalized(block) => {
              self.finalized = true;
              self.block = Some(block);
            }
            Future | Ready | Broadcast(_) => (),
            Retracted(_) => {
              // The transaction is back in the pool.  Might be included in a future block.
              self.block = None;
            }
            _ => {
              // Call failed to be included in a block or finalized.
              self.block = None;
              self.sub = None;
            }
          }
          self.status = Some(status);
          Ok(true)
        }
        Some(Err(err)) => {
          // Error waiting for an update.  Most likely the connection was closed.
          self.sub = None;
          Err(err)?
        }
      }
    } else {
      Ok(false)
    }
  }

  pub async fn events(&mut self) -> Result<Option<&EventRecords<Api::RuntimeEvent>>> {
    self.load_events().await?;
    Ok(self.events.as_ref())
  }

  async fn load_events(&mut self) -> Result<bool> {
    // Do nothing if we already have the events.
    if self.events.is_some() {
      return Ok(true);
    }

    // Make sure the transaction is in a block.
    let block_hash = if let Some(block) = self.block {
      block
    } else {
      match self.wait_in_block().await? {
        None => {
          // Still not in a block.
          return Ok(false);
        }
        Some(block) => block,
      }
    };

    // Find the extrinsic index of our transaction.
    let client = self.api.client();
    let idx = client
      .find_extrinsic_block_index(block_hash, self.tx_hash)
      .await?;

    if let Some(idx) = idx {
      // Get block events.
      let events = self.api.block_events(Some(block_hash)).await?;
      self.events = Some(EventRecords::from_vec(
        events,
        Some(Phase::ApplyExtrinsic(idx as u32)),
      ));
      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub fn status(&self) -> Option<&TransactionStatus> {
    self.status.as_ref()
  }

  pub async fn wait_in_block(&mut self) -> Result<Option<BlockHash>> {
    // Wait for call to be included in a block.
    while self.block.is_none() {
      if !self.next_status().await? {
        // No more updates available.
        return Ok(None);
      }
    }
    return Ok(self.block);
  }
}

pub struct Call<'api, Api: ChainApi> {
  api: &'api Api,
  call: Api::RuntimeCall,
}

impl<'api, Api: ChainApi> Call<'api, Api> {
  pub fn new(api: &'api Api, call: Api::RuntimeCall) -> Self {
    Self { api, call }
  }

  pub fn runtime_call(&self) -> &Api::RuntimeCall {
    &self.call
  }

  pub fn into_runtime_call(self) -> Api::RuntimeCall {
    self.call
  }

  pub fn encoded(&self) -> Encoded {
    let call = &self.call;
    call.into()
  }

  pub async fn submit_unsigned_and_watch(&self) -> Result<CallResults<'api, Api>> {
    Ok(
      self
        .submit_and_watch(ExtrinsicV4::unsigned(self.encoded()))
        .await?,
    )
  }

  pub async fn submit_and_watch(&self, xt: ExtrinsicV4) -> Result<CallResults<'api, Api>> {
    let (tx_hex, tx_hash) = xt.as_hex_and_hash();
    let status = self.api.client().submit_and_watch(tx_hex).await?;
    Ok(CallResults::new(self.api, status, tx_hash))
  }
}

impl<'api, Api: ChainApi> Encode for Call<'api, Api> {
  fn size_hint(&self) -> usize {
    self.call.size_hint()
  }
  fn encode_to<T: ::codec::Output + ?Sized>(&self, dest: &mut T) {
    self.call.encode_to(dest)
  }
}

impl<'api, Api: ChainApi> std::fmt::Debug for Call<'api, Api> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.call.fmt(f)
  }
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

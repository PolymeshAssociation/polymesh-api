use jsonrpsee::rpc_params;
use jsonrpsee::types::ParamsSer;
use jsonrpsee::core::client::Subscription;

use codec::Encode;

use sp_core::{
  sr25519, Pair,
};
use sp_runtime::generic::Era;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use frame_metadata::RuntimeMetadataPrefixed;
use sp_version::RuntimeVersion;

use crate::rpc::*;
use crate::block::*;
use crate::error::*;

pub trait ChainApi {
  type RuntimeCall: Clone + Encode + std::fmt::Debug;

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

  pub async fn submit_and_watch<'api, Api: ChainApi>(&mut self, call: &WrappedCall<'api, Api>) -> Result<Subscription<TransactionStatus>> {
    let client = call.api.client();
    // Query account nonce.
    if self.nonce == 0 {
        self.nonce = client.get_nonce(&self.account).await?.unwrap_or(0);
    }

    let encoded_call = call.encoded();
    let extra = Extra::new(Era::Immortal, self.nonce);
    let payload = SignedPayload::new(&encoded_call, &extra, client.get_signed_extra());

    let sig = payload.using_encoded(|p| self.pair.sign(p));

    let xt = ExtrinsicV4::signed(self.account.clone(), sig.into(), extra, encoded_call);

    let res = client.submit_and_watch(xt).await?;

    // Update nonce if the call was submitted.
    self.nonce += 1;

    Ok(res)
  }
}

pub struct WrappedCall<'api, Api: ChainApi> {
  api: &'api Api,
  call: Api::RuntimeCall,
}

impl<'api, Api: ChainApi> WrappedCall<'api, Api> {
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

  pub async fn submit_and_watch(&self) -> Result<Subscription<TransactionStatus>> {
    let xt = ExtrinsicV4::unsigned(self.encoded());

    Ok(self.api.client().submit_and_watch(xt).await?)
  }
}

impl<'api, Api: ChainApi> Encode for WrappedCall<'api, Api> {
  fn size_hint(&self) -> usize {
    self.call.size_hint()
  }
  fn encode_to<T: ::codec::Output + ?Sized>(&self, dest: &mut T) {
    self.call.encode_to(dest)
  }
}

impl<'api, Api: ChainApi> std::fmt::Debug for WrappedCall<'api, Api> {
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
    let runtime_version = rpc.get_runtime_version(None).await?;
    let metadata = rpc.get_metadata(None).await?;
    let genesis_hash = rpc.get_block_hash(0).await?;
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

  pub async fn get_nonce(&self, _account: &AccountId) -> Result<Option<u32>> {
    // TODO:
    Ok(Some(0))
  }

  pub async fn submit_and_watch(&self, xt: ExtrinsicV4) -> Result<Subscription<TransactionStatus>> {
    let xthex = xt.to_hex();
    Ok(self.rpc.subscribe(
      "author_submitAndWatchExtrinsic",
      rpc_params!(xthex),
      "author_unwatchExtrinsic",
    ).await?)
  }

  pub async fn request<'a, R>(
    &self,
    method: &'a str,
    params: Option<ParamsSer<'a>>,
  ) -> Result<R>
  where
    R: DeserializeOwned,
  {
    Ok(self.rpc.request(method, params).await?)
  }

  pub async fn batch_request<'a, R>(
    &self,
    batch: Vec<(&'a str, Option<ParamsSer<'a>>)>,
  ) -> Result<Vec<R>>
  where
    R: DeserializeOwned + Default + Clone,
  {
    Ok(self.rpc.batch_request(batch).await?)
  }

  pub async fn get_block_hash(&self, block_number: u32) -> Result<BlockHash> {
    Ok(self.rpc.get_block_hash(block_number).await?)
  }

  pub async fn get_block_runtime_version(&self, block: Option<BlockHash>) -> Result<RuntimeVersion> {
    Ok(self.rpc.get_runtime_version(block).await?)
  }

  pub async fn get_block_metadata(&self, block: Option<BlockHash>) -> Result<RuntimeMetadataPrefixed> {
    Ok(self.rpc.get_metadata(block).await?)
  }
}

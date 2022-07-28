use jsonrpsee::core::client::Subscription;

use codec::{Decode, Encode};
use sp_runtime::generic::Era;

use async_trait::async_trait;

use serde::{de::DeserializeOwned, ser::Serialize};

use crate::*;

pub trait CodecSerde:
  Clone + Encode + Decode + Serialize + DeserializeOwned + std::fmt::Debug
{
}

impl<T> CodecSerde for T where
  T: Clone + Encode + Decode + Serialize + DeserializeOwned + std::fmt::Debug
{
}

#[async_trait]
pub trait ChainApi {
  type RuntimeCall: CodecSerde;
  type RuntimeEvent: CodecSerde;

  async fn get_nonce(&self, account: AccountId) -> Result<u32>;

  async fn block_events(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Vec<EventRecord<Self::RuntimeEvent>>>;

  fn client(&self) -> &Client;
}

pub struct TransactionResults<'api, Api: ChainApi> {
  api: &'api Api,
  sub: Option<Subscription<TransactionStatus>>,
  tx_hash: TxHash,
  status: Option<TransactionStatus>,
  block: Option<BlockHash>,
  events: Option<EventRecords<Api::RuntimeEvent>>,
  finalized: bool,
}

impl<'api, Api: ChainApi> TransactionResults<'api, Api> {
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
  pub api: &'api Api,
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

  pub async fn submit_unsigned_and_watch(&self) -> Result<TransactionResults<'api, Api>> {
    Ok(
      self
        .submit_and_watch(ExtrinsicV4::unsigned(self.encoded()))
        .await?,
    )
  }

  pub async fn sign_submit_and_watch(
    &self,
    signer: &mut impl Signer,
  ) -> Result<TransactionResults<'api, Api>> {
    let client = self.api.client();
    let account = signer.account();
    // Query account nonce.
    let nonce = match signer.nonce() {
      Some(0) | None => self.api.get_nonce(account.clone()).await?,
      Some(nonce) => nonce,
    };

    let encoded_call = self.encoded();
    let extra = Extra::new(Era::Immortal, nonce);
    let payload = SignedPayload::new(&encoded_call, &extra, client.get_signed_extra());

    let payload = payload.encode();
    let sig = signer.sign(&payload[..]).await?;

    let xt = ExtrinsicV4::signed(account, sig, extra, encoded_call);

    let res = self.submit_and_watch(xt).await?;

    // Update nonce if the call was submitted.
    signer.set_nonce(nonce + 1);

    Ok(res)
  }

  pub async fn submit_and_watch(&self, xt: ExtrinsicV4) -> Result<TransactionResults<'api, Api>> {
    let (tx_hex, tx_hash) = xt.as_hex_and_hash();
    let status = self.api.client().submit_and_watch(tx_hex).await?;
    Ok(TransactionResults::new(self.api, status, tx_hash))
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

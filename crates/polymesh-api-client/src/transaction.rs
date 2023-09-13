use jsonrpsee::core::client::Subscription;

#[cfg(not(feature = "std"))]
use alloc::format;
use codec::{Decode, Encode};
use sp_std::prelude::*;

use async_trait::async_trait;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::*;

#[cfg(feature = "serde")]
pub trait RuntimeTraits:
  Clone + Encode + Decode + Serialize + DeserializeOwned + core::fmt::Debug
{
}

#[cfg(not(feature = "serde"))]
pub trait RuntimeTraits: Clone + Encode + Decode + core::fmt::Debug {}

#[cfg(feature = "serde")]
impl<T> RuntimeTraits for T where
  T: Clone + Encode + Decode + Serialize + DeserializeOwned + core::fmt::Debug
{
}

#[cfg(not(feature = "serde"))]
impl<T> RuntimeTraits for T where T: Clone + Encode + Decode + core::fmt::Debug {}

pub trait RuntimeEnumTraits: RuntimeTraits + EnumInfo {}

impl<T> RuntimeEnumTraits for T where T: RuntimeTraits + EnumInfo {}

pub trait EnumInfo: Into<&'static str> {
  fn as_name(&self) -> &'static str;
  fn as_docs(&self) -> &'static [&'static str];
  fn as_short_doc(&self) -> &'static str {
    self.as_docs()[0]
  }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExtrinsicResult<Api: ChainApi + ?Sized> {
  Success(Api::DispatchInfo),
  Failed(Api::DispatchInfo, Api::DispatchError),
}

impl<Api: ChainApi> ExtrinsicResult<Api> {
  pub fn is_success(&self) -> bool {
    match self {
      Self::Success(_) => true,
      Self::Failed(_, _) => false,
    }
  }

  pub fn is_failed(&self) -> bool {
    match self {
      Self::Success(_) => false,
      Self::Failed(_, _) => true,
    }
  }

  pub fn ok(&self) -> Result<()> {
    match self {
      Self::Success(_) => Ok(()),
      Self::Failed(_, err) => Err(Error::ExtrinsicError(format!("{}", err.as_short_doc()))),
    }
  }
}

#[async_trait]
pub trait ChainApi: Clone {
  type RuntimeCall: RuntimeEnumTraits;
  type RuntimeEvent: RuntimeEnumTraits;
  type DispatchInfo: RuntimeTraits;
  type DispatchError: RuntimeEnumTraits;

  async fn get_nonce(&self, account: AccountId) -> Result<u32>;

  async fn block_events(
    &self,
    block: Option<BlockHash>,
  ) -> Result<Vec<EventRecord<Self::RuntimeEvent>>>;

  fn event_to_extrinsic_result(
    event: &EventRecord<Self::RuntimeEvent>,
  ) -> Option<ExtrinsicResult<Self>>;

  fn events_to_extrinsic_result(
    events: &[EventRecord<Self::RuntimeEvent>],
  ) -> Option<ExtrinsicResult<Self>> {
    // Search backwards, since the event we want is normally the last.
    events
      .iter()
      .rev()
      .find_map(Self::event_to_extrinsic_result)
  }

  fn client(&self) -> &Client;

  /// Submit a signed/unsigned transaction, but don't wait for it to execute.
  async fn submit_and_watch(&self, xt: ExtrinsicV4) -> Result<TransactionResults<Self>> {
    let (tx_hex, tx_hash) = xt.as_hex_and_hash();
    let status = self.client().submit_and_watch(tx_hex).await?;
    Ok(TransactionResults::new(self, status, tx_hash))
  }
}

pub struct TransactionResults<Api: ChainApi> {
  api: Api,
  sub: Option<Subscription<TransactionStatus>>,
  tx_hash: TxHash,
  status: Option<TransactionStatus>,
  block: Option<BlockHash>,
  events: Option<EventRecords<Api::RuntimeEvent>>,
  extrinsic_result: Option<ExtrinsicResult<Api>>,
  finalized: bool,
}

impl<Api: ChainApi> TransactionResults<Api> {
  pub fn new(api: &Api, sub: Subscription<TransactionStatus>, tx_hash: TxHash) -> Self {
    Self {
      api: api.clone(),
      sub: Some(sub),
      tx_hash,
      status: None,
      block: None,
      events: None,
      extrinsic_result: None,
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

  pub async fn extrinsic_result(&mut self) -> Result<Option<&ExtrinsicResult<Api>>> {
    self.load_events().await?;
    Ok(self.extrinsic_result.as_ref())
  }

  pub async fn ok(&mut self) -> Result<()> {
    match self.extrinsic_result().await? {
      Some(res) => res.ok(),
      None => Err(Error::ExtrinsicError(
        "Failed to get extrinsic results".into(),
      )),
    }
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
      let block_events = self.api.block_events(Some(block_hash)).await?;
      let events = EventRecords::from_vec(block_events, Some(Phase::ApplyExtrinsic(idx as u32)));
      self.extrinsic_result = Api::events_to_extrinsic_result(events.0.as_slice());
      self.events = Some(events);
      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub fn status(&self) -> Option<&TransactionStatus> {
    self.status.as_ref()
  }

  /// Wait for the transaction to be included in a block.
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

  /// Wait for the transaction to be finalized.
  pub async fn wait_finalized(&mut self) -> Result<Option<BlockHash>> {
    // Wait for call to be included in a block.
    while !self.finalized {
      if !self.next_status().await? {
        // No more updates available.
        return Ok(None);
      }
    }
    return Ok(self.block);
  }
}

pub struct Call<Api: ChainApi> {
  pub api: Api,
  call: Api::RuntimeCall,
}

impl<Api: ChainApi> Call<Api> {
  pub fn new(api: &Api, call: Api::RuntimeCall) -> Self {
    Self {
      api: api.clone(),
      call,
    }
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

  /// Submit the transaction unsigned.
  pub async fn submit_unsigned_and_watch(&self) -> Result<TransactionResults<Api>> {
    Ok(
      self
        .submit_raw_xt_and_watch(ExtrinsicV4::unsigned(self.encoded()))
        .await?,
    )
  }

  /// Prepare a transaction for offline signing.
  pub async fn prepare(&self, account: AccountId) -> Result<PreparedTransaction> {
    let client = self.api.client();
    // Query account nonce.
    let nonce = self.api.get_nonce(account).await?;

    let encoded_call = self.encoded();
    let extra = Extra::new(Era::Immortal, nonce);
    let additional = client.get_signed_extra();
    Ok(PreparedTransaction::new(account, additional, extra, encoded_call))
  }

  /// Sign, submit and execute the transaction.
  pub async fn execute(&self, signer: &mut impl Signer) -> Result<TransactionResults<Api>> {
    // Sign and submit transaction.
    let mut res = self.submit_and_watch(signer).await?;
    // Wait for transaction to be included in a block.
    res.ok().await?;
    // Transaction successful.
    Ok(res)
  }

  /// Sign and submit the transaction, but don't wait for it to execute.
  ///
  /// The return values can be used to wait for transaction to execute and get the results.
  pub async fn submit_and_watch(
    &self,
    signer: &mut impl Signer,
  ) -> Result<TransactionResults<Api>> {
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

    let res = self.submit_raw_xt_and_watch(xt).await?;

    // Update nonce if the call was submitted.
    signer.set_nonce(nonce + 1);

    Ok(res)
  }

  /// Submit a signed/unsigned transaction, but don't wait for it to execute.
  ///
  /// You most likely want to uses either [`Self::execute`] or [`Self::submit_and_watch`]
  /// not this method.
  pub async fn submit_raw_xt_and_watch(&self, xt: ExtrinsicV4) -> Result<TransactionResults<Api>> {
    let (tx_hex, tx_hash) = xt.as_hex_and_hash();
    let status = self.api.client().submit_and_watch(tx_hex).await?;
    Ok(TransactionResults::new(&self.api, status, tx_hash))
  }
}

impl<Api: ChainApi> Encode for Call<Api> {
  fn size_hint(&self) -> usize {
    self.call.size_hint()
  }
  fn encode_to<T: ::codec::Output + ?Sized>(&self, dest: &mut T) {
    self.call.encode_to(dest)
  }
}

impl<Api: ChainApi> core::fmt::Debug for Call<Api> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.call.fmt(f)
  }
}

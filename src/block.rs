use codec::{Compact, Decode, Encode, Output};

use sp_core::{hashing::blake2_256, H256};
use sp_runtime::{
  generic::{self, Era},
  traits, MultiAddress, MultiSignature,
};

use serde::{Deserialize, Serialize};

use crate::*;

pub type TxHash = H256;
pub type BlockHash = H256;
pub type Header = generic::Header<u32, traits::BlakeTwo256>;

pub type AccountId = sp_runtime::AccountId32;
pub type GenericAddress = MultiAddress<AccountId, ()>;

pub type AdditionalSigned = (u32, u32, BlockHash, BlockHash, (), (), ());

#[derive(Clone, Debug, Encode, Decode)]
pub struct Extra(Era, Compact<u32>, Compact<u128>);

impl Extra {
  pub fn new(era: Era, nonce: u32) -> Self {
    Self(era, nonce.into(), 0u128.into())
  }

  pub fn nonce(&self) -> u32 {
    self.1 .0
  }

  pub fn tip(&self) -> u128 {
    self.2 .0
  }
}

pub struct Encoded(Vec<u8>);

impl<T: Encode> From<&T> for Encoded {
  fn from(other: &T) -> Self {
    Self(other.encode())
  }
}

impl Encode for Encoded {
  fn size_hint(&self) -> usize {
    self.0.len()
  }
  fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
    dest.write(&self.0);
  }
}

pub struct SignedPayload<'a>((&'a Encoded, &'a Extra, AdditionalSigned));

impl<'a> SignedPayload<'a> {
  pub fn new(call: &'a Encoded, extra: &'a Extra, additional: AdditionalSigned) -> Self {
    Self((call, extra, additional))
  }
}

impl<'a> Encode for SignedPayload<'a> {
  fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
    self.0.using_encoded(|payload| {
      if payload.len() > 256 {
        f(&blake2_256(payload)[..])
      } else {
        f(payload)
      }
    })
  }
}

/// Current version of the `UncheckedExtrinsic` format.
pub const EXTRINSIC_VERSION: u8 = 4;

pub struct ExtrinsicV4 {
  pub signature: Option<(GenericAddress, MultiSignature, Extra)>,
  pub call: Encoded,
}

impl ExtrinsicV4 {
  pub fn tx_hash(tx: &[u8]) -> TxHash {
    H256(blake2_256(tx))
  }

  pub fn signed(account: AccountId, sig: MultiSignature, extra: Extra, call: Encoded) -> Self {
    Self {
      signature: Some((GenericAddress::from(account), sig, extra)),
      call,
    }
  }

  pub fn unsigned(call: Encoded) -> Self {
    Self {
      signature: None,
      call,
    }
  }

  pub fn as_hex_and_hash(&self) -> (String, TxHash) {
    let tx = self.encode();
    let mut tx_hex = hex::encode(tx);
    tx_hex.insert_str(0, "0x");
    let tx_hash = Self::tx_hash(tx_hex.as_bytes());
    (tx_hex, tx_hash)
  }

  pub fn to_hex(&self) -> String {
    let mut hex = hex::encode(self.encode());
    hex.insert_str(0, "0x");
    hex
  }
}

impl Encode for ExtrinsicV4 {
  fn encode(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(512);

    // 1 byte version id and signature if signed.
    match &self.signature {
      Some(sig) => {
        buf.push(EXTRINSIC_VERSION | 0b1000_0000);
        sig.encode_to(&mut buf);
      }
      None => {
        buf.push(EXTRINSIC_VERSION & 0b0111_1111);
      }
    }
    self.call.encode_to(&mut buf);

    buf.encode()
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccountInfo {
  pub nonce: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
  Future,
  Ready,
  Broadcast(Vec<String>),
  InBlock(BlockHash),
  Retracted(BlockHash),
  FinalityTimeout(BlockHash),
  Finalized(BlockHash),
  Usurped(TxHash),
  Dropped,
  Invalid,
}

pub type SignedBlock = generic::SignedBlock<Block>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
  extrinsics: Vec<String>,
  header: Header,
}

impl Block {
  pub fn find_extrinsic(&self, xt_hash: TxHash) -> Option<usize> {
    // TODO: Add caching of blocks with extrinsic hashes.
    self.extrinsics.iter().position(|xt| {
      let hash = ExtrinsicV4::tx_hash(xt.as_bytes());
      hash == xt_hash
    })
  }
  pub fn parent(&self) -> BlockHash {
    self.header.parent_hash
  }

  pub fn state_root(&self) -> BlockHash {
    self.header.state_root
  }

  pub fn extrinsics_root(&self) -> BlockHash {
    self.header.extrinsics_root
  }

  pub fn block_number(&self) -> i64 {
    self.header.number as i64
  }

  pub fn to_string(&self) -> String {
    format!("{:?}", self)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, PartialEq, Eq)]
pub enum Phase {
  ApplyExtrinsic(u32),
  Finalization,
  Initialization,
}

#[derive(Clone, Debug, Serialize, Decode)]
pub struct EventRecord<Event: CodecSerde> {
  pub phase: Phase,
  pub event: Event,
  pub topics: Vec<BlockHash>,
}

impl<Event: CodecSerde> EventRecord<Event> {
  pub fn to_string(&self) -> String {
    format!("{:#?}", self)
  }
}

#[derive(Clone, Debug, Serialize, Decode, Default)]
pub struct EventRecords<Event: CodecSerde>(Vec<EventRecord<Event>>);

impl<Event: CodecSerde> EventRecords<Event> {
  pub fn from_vec(mut events: Vec<EventRecord<Event>>, filter: Option<Phase>) -> Self {
    if let Some(filter) = filter {
      events.retain(|ev| ev.phase == filter);
    }
    Self(events)
  }

  pub fn to_string(&self) -> String {
    format!("{:#?}", self.0)
  }
}

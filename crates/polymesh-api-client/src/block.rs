use codec::{Compact, Decode, Encode, Output};

#[cfg(all(feature = "std", feature = "type_info"))]
use scale_info::TypeInfo;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use sp_core::{hashing::blake2_256, H256};
use sp_runtime::{ConsensusEngineId, MultiSignature};
use sp_std::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::basic_types::{AccountId, GenericAddress};
use crate::*;

pub type TxHash = H256;
pub type BlockHash = H256;
pub type BlockNumber = u32;

#[cfg(feature = "serde")]
pub mod block_number {
  use super::BlockNumber;
  use sp_core::U256;

  pub fn serialize<S>(num: &BlockNumber, s: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let num = U256::from(*num);
    serde::Serialize::serialize(&num, s)
  }

  pub fn deserialize<'de, D>(d: D) -> Result<BlockNumber, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let num: U256 = serde::Deserialize::deserialize(d)?;
    Ok(num.as_u32())
  }
}

#[derive(Clone, Debug, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Header {
  pub parent_hash: BlockHash,
  #[cfg_attr(feature = "serde", serde(with = "block_number"))]
  #[codec(compact)]
  pub number: BlockNumber,
  pub state_root: BlockHash,
  pub extrinsics_root: BlockHash,
  pub digest: Digest,
}

impl Header {
  pub fn hash(&self) -> BlockHash {
    H256(self.using_encoded(blake2_256))
  }
}

impl From<Header> for sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256> {
  fn from(header: Header) -> Self {
    let logs = header
      .digest
      .logs
      .into_iter()
      .map(|item| item.into())
      .collect();
    Self {
      parent_hash: header.parent_hash,
      number: header.number,
      state_root: header.state_root,
      extrinsics_root: header.extrinsics_root,
      digest: sp_runtime::generic::Digest { logs },
    }
  }
}

#[derive(Clone, Debug, Default, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Digest {
  pub logs: Vec<DigestItem>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RawDigestItem"))]
#[cfg_attr(feature = "serde", serde(into = "RawDigestItem"))]
pub enum DigestItem {
  PreRuntime(ConsensusEngineId, Vec<u8>),
  Consensus(ConsensusEngineId, Vec<u8>),
  Seal(ConsensusEngineId, Vec<u8>),
  Other(Vec<u8>),
  RuntimeEnvironmentUpdated,
}

impl Encode for DigestItem {
  fn encode_to<T: Output + ?Sized>(&self, output: &mut T) {
    let runtime_era: sp_runtime::generic::DigestItem = self.clone().into();
    runtime_era.encode_to(output)
  }
}

impl Decode for DigestItem {
  fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
    let runtime_era = sp_runtime::generic::DigestItem::decode(input)?;
    Ok(runtime_era.into())
  }
}

impl From<sp_runtime::generic::DigestItem> for DigestItem {
  fn from(r_item: sp_runtime::generic::DigestItem) -> Self {
    use sp_runtime::generic::DigestItem::*;
    match r_item {
      PreRuntime(id, data) => Self::PreRuntime(id, data),
      Consensus(id, data) => Self::Consensus(id, data),
      Seal(id, data) => Self::Seal(id, data),
      Other(data) => Self::Other(data),
      RuntimeEnvironmentUpdated => Self::RuntimeEnvironmentUpdated,
    }
  }
}

impl From<DigestItem> for sp_runtime::generic::DigestItem {
  fn from(item: DigestItem) -> Self {
    match item {
      DigestItem::PreRuntime(id, data) => Self::PreRuntime(id, data),
      DigestItem::Consensus(id, data) => Self::Consensus(id, data),
      DigestItem::Seal(id, data) => Self::Seal(id, data),
      DigestItem::Other(data) => Self::Other(data),
      DigestItem::RuntimeEnvironmentUpdated => Self::RuntimeEnvironmentUpdated,
    }
  }
}

impl TryFrom<RawDigestItem> for DigestItem {
  type Error = crate::Error;

  fn try_from(raw: RawDigestItem) -> Result<Self, Self::Error> {
    let item = DigestItem::decode(&mut &raw.0[..])?;
    Ok(item.into())
  }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawDigestItem(
  #[cfg_attr(feature = "serde", serde(with = "impl_serde::serialize"))] pub Vec<u8>,
);

impl From<DigestItem> for RawDigestItem {
  fn from(item: DigestItem) -> Self {
    Self(item.encode())
  }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StorageData(
  #[cfg_attr(feature = "serde", serde(with = "impl_serde::serialize"))] pub Vec<u8>,
);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StorageKey(
  #[cfg_attr(feature = "serde", serde(with = "impl_serde::serialize"))] pub Vec<u8>,
);

#[derive(Clone, Debug, Default, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdditionalSigned {
  pub spec_version: u32,
  pub tx_version: u32,
  pub genesis_hash: BlockHash,
  pub current_hash: BlockHash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
pub enum Era {
  Immortal,
  Mortal(u64, u64),
}

impl Era {
  pub fn mortal(current: BlockNumber, period: Option<u64>) -> Self {
    let period = period.unwrap_or(64);
    sp_runtime::generic::Era::mortal(period, current.into()).into()
  }

  pub fn immortal() -> Self {
    Self::Immortal
  }
}

impl Encode for Era {
  fn encode_to<T: Output + ?Sized>(&self, output: &mut T) {
    let runtime_era: sp_runtime::generic::Era = self.clone().into();
    runtime_era.encode_to(output)
  }
}

impl Decode for Era {
  fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
    let runtime_era = sp_runtime::generic::Era::decode(input)?;
    Ok(runtime_era.into())
  }
}

impl From<sp_runtime::generic::Era> for Era {
  fn from(e: sp_runtime::generic::Era) -> Self {
    match e {
      sp_runtime::generic::Era::Immortal => Self::Immortal,
      sp_runtime::generic::Era::Mortal(period, phase) => Self::Mortal(period, phase),
    }
  }
}

impl From<Era> for sp_runtime::generic::Era {
  fn from(e: Era) -> Self {
    match e {
      Era::Immortal => Self::Immortal,
      Era::Mortal(period, phase) => Self::Mortal(period, phase),
    }
  }
}

#[derive(Clone, Debug, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Extra(sp_runtime::generic::Era, Compact<u32>, Compact<u128>);

impl Extra {
  pub fn new(era: Era, nonce: u32) -> Self {
    Self(era.into(), nonce.into(), 0u128.into())
  }

  pub fn nonce(&self) -> u32 {
    self.1 .0
  }

  pub fn tip(&self) -> u128 {
    self.2 .0
  }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Encoded(
  #[cfg_attr(feature = "serde", serde(with = "impl_serde::serialize"))] pub Vec<u8>,
);

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

impl Decode for Encoded {
  fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
    if let Some(len) = input.remaining_len()? {
      let mut data = vec![0u8; len];
      input.read(&mut data.as_mut_slice())?;
      Ok(Self(data))
    } else {
      let mut data = Vec::new();
      while let Ok(b) = input.read_byte() {
        data.push(b);
      }
      Ok(Self(data))
    }
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

/// PreparedTransaction holds all data needed to sign a transaction.
///
/// This can be used for offline signers.
#[derive(Clone, Debug, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PreparedTransaction {
  pub account: AccountId,
  pub additional: AdditionalSigned,
  pub extra: Extra,
  pub call: Encoded,
}

impl PreparedTransaction {
  pub fn new(
    account: AccountId,
    additional: AdditionalSigned,
    extra: Extra,
    call: Encoded,
  ) -> Self {
    Self {
      account,
      additional,
      extra,
      call,
    }
  }

  pub async fn sign(self, signer: &mut impl Signer) -> Result<ExtrinsicV4> {
    // Ensure the signer's account matches the transaction.
    let account = signer.account();
    if account != self.account {
      use sp_core::crypto::Ss58Codec;
      let version = 12u16.into(); // Polymesh
      let a1 = account.to_ss58check_with_version(version);
      let a2 = self.account.to_ss58check_with_version(version);
      return Err(Error::WrongSignerAccount(a1, a2));
    }
    let payload = SignedPayload::new(&self.call, &self.extra, self.additional);
    let payload = payload.encode();
    let sig = signer.sign(&payload[..]).await?;

    let xt = ExtrinsicV4::signed(self.account, sig, self.extra, self.call);
    Ok(xt)
  }
}

/// Current version of the `UncheckedExtrinsic` format.
pub const EXTRINSIC_VERSION: u8 = 4;

#[derive(Clone, Debug)]
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
    let tx_hash = Self::tx_hash(tx.as_slice());
    let mut tx_hex = hex::encode(tx);
    tx_hex.insert_str(0, "0x");
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

impl Decode for ExtrinsicV4 {
  fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
    // Decode Vec length.
    let _len: Compact<u32> = Decode::decode(input)?;
    // Version and signed flag.
    let version = input.read_byte()?;
    let is_signed = version & 0b1000_0000 != 0;
    if (version & 0b0111_1111) != EXTRINSIC_VERSION {
      Err("Invalid EXTRINSIC_VERSION")?;
    }

    let signature = if is_signed {
      let sig: (GenericAddress, MultiSignature, Extra) = Decode::decode(input)?;
      Some(sig)
    } else {
      None
    };

    Ok(Self {
      signature,
      call: Decode::decode(input)?,
    })
  }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct AccountInfo {
  pub nonce: u32,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SignedBlock {
  pub block: Block,
  // TODO: Add Justifications field.
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
  extrinsics: Vec<Encoded>,
  header: Header,
}

impl Block {
  pub fn find_extrinsic(&self, xt_hash: TxHash) -> Option<usize> {
    // TODO: Add caching of blocks with extrinsic hashes.
    self
      .extrinsics
      .iter()
      .position(|xt| ExtrinsicV4::tx_hash(xt.0.as_slice()) == xt_hash)
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

  pub fn block_number(&self) -> BlockNumber {
    self.header.number
  }

  pub fn to_string(&self) -> String {
    format!("{:?}", self)
  }
}

#[derive(Clone, Debug, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Phase {
  ApplyExtrinsic(u32),
  Finalization,
  Initialization,
}

#[derive(Clone, Debug, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EventRecord<Event: RuntimeEnumTraits> {
  pub phase: Phase,
  pub event: Event,
  pub topics: Vec<BlockHash>,
}

impl<Event: RuntimeEnumTraits> EventRecord<Event> {
  pub fn name(&self) -> &'static str {
    self.event.as_name()
  }

  pub fn short_doc(&self) -> &'static str {
    self.event.as_short_doc()
  }

  pub fn docs(&self) -> &'static [&'static str] {
    self.event.as_docs()
  }

  pub fn to_string(&self) -> String {
    format!("{:#?}", self)
  }
}

#[derive(Clone, Debug, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EventRecords<Event: RuntimeEnumTraits>(pub Vec<EventRecord<Event>>);

impl<Event: RuntimeEnumTraits> EventRecords<Event> {
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

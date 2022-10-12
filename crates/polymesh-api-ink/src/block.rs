#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec, vec::Vec};

use codec::{Decode, Encode, Output};

#[cfg(feature = "std")]
use scale_info::TypeInfo;

use primitive_types::H256;

use crate::*;

pub type TxHash = H256;
pub type BlockHash = H256;
pub type BlockNumber = u32;

#[derive(Clone, Debug)]
pub struct StorageData(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct StorageKey(pub Vec<u8>);

// TODO: Fix Encode/Decode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum Era {
  Immortal,
  Mortal(u64, u64),
}

#[derive(Clone, Debug)]
pub struct Encoded(pub Vec<u8>);

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

#[derive(Clone, Debug)]
pub struct AccountInfo {
  pub nonce: u32,
}

#[derive(Clone, Debug, Decode, PartialEq, Eq)]
pub enum Phase {
  ApplyExtrinsic(u32),
  Finalization,
  Initialization,
}

#[derive(Clone, Debug, Decode)]
pub struct EventRecord<Event: RuntimeTraits> {
  pub phase: Phase,
  pub event: Event,
  pub topics: Vec<BlockHash>,
}

#[derive(Clone, Debug, Decode, Default)]
pub struct EventRecords<Event: RuntimeTraits>(pub Vec<EventRecord<Event>>);

impl<Event: RuntimeTraits> EventRecords<Event> {
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

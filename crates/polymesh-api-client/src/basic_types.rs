use codec::{Compact, CompactAs, Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Re-export some basic crates.
pub use frame_metadata;
pub use sp_arithmetic;
pub use sp_core;
pub use sp_runtime;

// Re-impl `per_things` to support serde
pub mod per_things {
  use super::*;

  #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
  #[cfg_attr(feature = "std", derive(Hash))]
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  pub struct Perbill(pub u32);

  impl CompactAs for Perbill {
    type As = u32;

    fn encode_as(&self) -> &Self::As {
      &self.0
    }

    fn decode_from(v: Self::As) -> Result<Self, codec::Error> {
      Ok(Self(v))
    }
  }

  impl From<Compact<Self>> for Perbill {
    fn from(c: Compact<Self>) -> Self {
      c.0
    }
  }

  impl From<sp_arithmetic::per_things::Perbill> for Perbill {
    fn from(p: sp_arithmetic::per_things::Perbill) -> Self {
      Self(p.deconstruct())
    }
  }

  #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
  #[cfg_attr(feature = "std", derive(Hash))]
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  pub struct PerU16(pub u16);

  impl CompactAs for PerU16 {
    type As = u16;

    fn encode_as(&self) -> &Self::As {
      &self.0
    }

    fn decode_from(v: Self::As) -> Result<Self, codec::Error> {
      Ok(Self(v))
    }
  }

  impl From<Compact<Self>> for PerU16 {
    fn from(c: Compact<Self>) -> Self {
      c.0
    }
  }

  impl From<sp_arithmetic::per_things::PerU16> for PerU16 {
    fn from(p: sp_arithmetic::per_things::PerU16) -> Self {
      Self(p.deconstruct())
    }
  }

  #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
  #[cfg_attr(feature = "std", derive(Hash))]
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  pub struct Permill(pub u32);

  impl CompactAs for Permill {
    type As = u32;

    fn encode_as(&self) -> &Self::As {
      &self.0
    }

    fn decode_from(v: Self::As) -> Result<Self, codec::Error> {
      Ok(Self(v))
    }
  }

  impl From<Compact<Self>> for Permill {
    fn from(c: Compact<Self>) -> Self {
      c.0
    }
  }

  impl From<sp_arithmetic::per_things::Permill> for Permill {
    fn from(p: sp_arithmetic::per_things::Permill) -> Self {
      Self(p.deconstruct())
    }
  }

  #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
  #[cfg_attr(feature = "std", derive(Hash))]
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  pub struct Percent(pub u8);

  impl CompactAs for Percent {
    type As = u8;

    fn encode_as(&self) -> &Self::As {
      &self.0
    }

    fn decode_from(v: Self::As) -> Result<Self, codec::Error> {
      Ok(Self(v))
    }
  }

  impl From<Compact<Self>> for Percent {
    fn from(c: Compact<Self>) -> Self {
      c.0
    }
  }

  impl From<sp_arithmetic::per_things::Percent> for Percent {
    fn from(p: sp_arithmetic::per_things::Percent) -> Self {
      Self(p.deconstruct())
    }
  }
}

// Re-impl MultiAddress to support serde
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MultiAddress<AccountId, AccountIndex> {
  /// It's an account ID (pubkey).
  Id(AccountId),
  /// It's an account index.
  Index(#[codec(compact)] AccountIndex),
  /// It's some arbitrary raw bytes.
  Raw(Vec<u8>),
  /// It's a 32 byte representation.
  Address32([u8; 32]),
  /// Its a 20 byte representation.
  Address20([u8; 20]),
}

impl<AccountId: Clone, AccountIndex> From<&AccountId> for MultiAddress<AccountId, AccountIndex> {
  fn from(other: &AccountId) -> Self {
    Self::Id(other.clone())
  }
}

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> {
  fn from(other: AccountId) -> Self {
    Self::Id(other)
  }
}

impl<AccountIndex> From<sp_runtime::AccountId32> for MultiAddress<AccountId, AccountIndex> {
  fn from(other: sp_runtime::AccountId32) -> Self {
    Self::Id(other.into())
  }
}

impl<AccountId, AccountIndex> From<sp_runtime::MultiAddress<AccountId, AccountIndex>>
  for MultiAddress<AccountId, AccountIndex>
{
  fn from(other: sp_runtime::MultiAddress<AccountId, AccountIndex>) -> Self {
    match other {
      sp_runtime::MultiAddress::Id(v) => Self::Id(v),
      sp_runtime::MultiAddress::Index(v) => Self::Index(v),
      sp_runtime::MultiAddress::Raw(v) => Self::Raw(v),
      sp_runtime::MultiAddress::Address32(v) => Self::Address32(v),
      sp_runtime::MultiAddress::Address20(v) => Self::Address20(v),
    }
  }
}

impl<AccountId: Clone, AccountIndex: Clone> From<&sp_runtime::MultiAddress<AccountId, AccountIndex>>
  for MultiAddress<AccountId, AccountIndex>
{
  fn from(other: &sp_runtime::MultiAddress<AccountId, AccountIndex>) -> Self {
    Self::from(other.clone())
  }
}

#[derive(
  Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, Serialize, Deserialize,
)]
pub struct AccountId(pub [u8; 32]);

impl From<[u8; 32]> for AccountId {
  fn from(p: [u8; 32]) -> Self {
    Self(p)
  }
}

impl From<sp_core::sr25519::Public> for AccountId {
  fn from(p: sp_core::sr25519::Public) -> Self {
    p.0.into()
  }
}

impl From<sp_core::ed25519::Public> for AccountId {
  fn from(p: sp_core::ed25519::Public) -> Self {
    p.0.into()
  }
}

impl From<sp_runtime::AccountId32> for AccountId {
  fn from(id: sp_runtime::AccountId32) -> Self {
    Self(*id.as_ref())
  }
}

impl From<&sp_runtime::AccountId32> for AccountId {
  fn from(id: &sp_runtime::AccountId32) -> Self {
    Self(*id.as_ref())
  }
}

impl From<AccountId> for sp_runtime::AccountId32 {
  fn from(id: AccountId) -> Self {
    Self::new(id.0)
  }
}

impl From<&AccountId> for sp_runtime::AccountId32 {
  fn from(id: &AccountId) -> Self {
    Self::new(id.0)
  }
}

pub type GenericAddress = MultiAddress<AccountId, ()>;

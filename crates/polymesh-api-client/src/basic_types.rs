use codec::{Compact, CompactAs, Decode, Encode};

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "std", feature = "type_info"))]
use scale_info::TypeInfo;

use sp_core::crypto::Ss58Codec;
use sp_std::prelude::*;
#[cfg(not(feature = "std"))]
use alloc::{
  format,
  string::String,
};

// Re-export some basic crates.
pub use frame_metadata;

pub use sp_core;

pub use sp_core::hashing;

pub use sp_weights;

// Re-impl `per_things` to support serde
pub mod per_things {
  use super::*;

  #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
  #[cfg_attr(feature = "std", derive(Hash))]
  #[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
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
  #[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
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
  #[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
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
  #[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
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
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
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

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
pub struct AccountId(pub [u8; 32]);

impl AccountId {
  pub fn to_hex(&self) -> String {
    format!("0x{}", hex::encode(&self.0))
  }
}

// TODO: re-implement ss58 for `no_std`
#[cfg(not(feature = "std"))]
impl AccountId {
  pub fn to_ss58check(&self) -> String {
    format!("0x{}", hex::encode(&self.0))
  }
}

impl Ss58Codec for AccountId {}

impl fmt::Display for AccountId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_ss58check())
  }
}

impl fmt::Debug for AccountId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let ss58 = self.to_ss58check();
    let h = hex::encode(&self.0);
    write!(f, "0x{} ({}...)", h, &ss58[0..8])
  }
}

#[cfg(feature = "serde")]
impl Serialize for AccountId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    let h = format!("0x{}", hex::encode(&self.0));
    serializer.serialize_str(&h)
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for AccountId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    let h: &str = Deserialize::deserialize(deserializer)?;
    let mut id = AccountId::default();
    let off = if h.starts_with("0x") { 2 } else { 0 };
    hex::decode_to_slice(h, &mut id.0[off..]).map_err(|e| serde::de::Error::custom(e))?;
    Ok(id)
  }
}

impl<'a> TryFrom<&'a [u8]> for AccountId {
  type Error = ();

  fn try_from(x: &'a [u8]) -> Result<Self, ()> {
    Ok(AccountId(x.try_into().map_err(|_| ())?))
  }
}

impl AsMut<[u8; 32]> for AccountId {
  fn as_mut(&mut self) -> &mut [u8; 32] {
    &mut self.0
  }
}

impl AsMut<[u8]> for AccountId {
  fn as_mut(&mut self) -> &mut [u8] {
    &mut self.0[..]
  }
}

impl AsRef<[u8; 32]> for AccountId {
  fn as_ref(&self) -> &[u8; 32] {
    &self.0
  }
}

impl AsRef<[u8]> for AccountId {
  fn as_ref(&self) -> &[u8] {
    &self.0[..]
  }
}

impl sp_core::ByteArray for AccountId {
  const LEN: usize = 32;
}

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

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
pub struct IdentityId(pub [u8; 32]);

impl fmt::Display for IdentityId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
  }
}

impl fmt::Debug for IdentityId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
  }
}

#[cfg(feature = "serde")]
impl Serialize for IdentityId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    let h = format!("0x{}", hex::encode(&self.0));
    serializer.serialize_str(&h)
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for IdentityId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    let h: &str = Deserialize::deserialize(deserializer)?;
    let mut id = IdentityId::default();
    let off = if h.starts_with("0x") { 2 } else { 0 };
    hex::decode_to_slice(h, &mut id.0[off..]).map_err(|e| serde::de::Error::custom(e))?;
    Ok(id)
  }
}

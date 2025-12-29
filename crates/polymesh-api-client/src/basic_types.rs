use codec::{Compact, CompactAs, Decode, Encode};

use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, ser, Deserialize, Serialize};

#[cfg(all(feature = "std", feature = "type_info"))]
use scale_info::TypeInfo;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use sp_core::crypto::Ss58Codec;
use sp_std::prelude::*;

// Re-export some basic crates.
pub use frame_metadata;

pub use sp_core;

pub use sp_core::hashing;

pub use sp_weights;

pub use sp_runtime::MultiSignature;

// Re-impl `OldWeight`
#[derive(
  Clone, Copy, Debug, Encode, Decode, CompactAs, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct OldWeight(pub u64);

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

#[cfg(not(feature = "serde"))]
pub trait AccountTraits: Clone + Encode + Decode + Default + FromStr {}

#[cfg(not(feature = "serde"))]
impl<T> AccountTraits for T where T: Clone + Encode + Decode + Default + FromStr {}

#[cfg(feature = "serde")]
pub trait AccountTraits:
  Clone + Encode + Decode + Default + FromStr + ser::Serialize + DeserializeOwned
{
}

#[cfg(feature = "serde")]
impl<T> AccountTraits for T where
  T: Clone + Encode + Decode + Default + FromStr + ser::Serialize + DeserializeOwned
{
}

// Re-impl MultiAddress to support serde
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Hash))]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
pub enum MultiAddress<AccountId: AccountTraits, AccountIndex: AccountTraits> {
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

impl<AccountId: AccountTraits, AccountIndex: AccountTraits> From<&AccountId>
  for MultiAddress<AccountId, AccountIndex>
{
  fn from(other: &AccountId) -> Self {
    Self::Id(other.clone())
  }
}

impl<AccountId: AccountTraits, AccountIndex: AccountTraits> From<AccountId>
  for MultiAddress<AccountId, AccountIndex>
{
  fn from(other: AccountId) -> Self {
    Self::Id(other)
  }
}

impl<AccountIndex: AccountTraits> From<sp_runtime::AccountId32>
  for MultiAddress<AccountId, AccountIndex>
{
  fn from(other: sp_runtime::AccountId32) -> Self {
    Self::Id(other.into())
  }
}

impl<AccountId: AccountTraits, AccountIndex: AccountTraits>
  From<sp_runtime::MultiAddress<AccountId, AccountIndex>>
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

impl<AccountId: AccountTraits, AccountIndex: AccountTraits>
  From<&sp_runtime::MultiAddress<AccountId, AccountIndex>>
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

impl FromStr for AccountId {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.len() {
      66 if s.starts_with("0x") => {
        let mut id = AccountId::default();
        hex::decode_to_slice(&s[2..], &mut id.0)?;
        Ok(id)
      }
      64 => {
        let mut id = AccountId::default();
        hex::decode_to_slice(&s[0..], &mut id.0)?;
        Ok(id)
      }
      _ => Ok(AccountId::from_ss58check(s)?),
    }
  }
}

#[cfg(not(feature = "serde"))]
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

pub type GenericAddress = MultiAddress<AccountId, u32>;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
  feature = "utoipa",
  schema(value_type = String, format = Binary, examples("0x0600000000000000000000000000000000000000000000000000000000000000"))
)]
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

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(all(feature = "std", feature = "type_info"), derive(TypeInfo))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
  feature = "utoipa",
  schema(value_type = String, format = Binary, examples("67e55044-10b1-426f-9247-bb680e5fe0c8"))
)]
pub struct AssetId(pub [u8; 16]);

impl AssetId {
  pub fn as_uuid(&self) -> uuid::Uuid {
    uuid::Uuid::from_bytes(self.0)
  }
}

impl fmt::Display for AssetId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_uuid().fmt(f)
  }
}

impl fmt::Debug for AssetId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_uuid().fmt(f)
  }
}

impl From<uuid::Uuid> for AssetId {
  fn from(uuid: uuid::Uuid) -> Self {
    Self(uuid.into_bytes())
  }
}

impl FromStr for AssetId {
  type Err = uuid::Error;

  fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
    let uuid = uuid::Uuid::parse_str(uuid_str)?;
    Ok(uuid.into())
  }
}

impl TryFrom<&'_ str> for AssetId {
  type Error = uuid::Error;

  fn try_from(uuid_str: &'_ str) -> Result<Self, Self::Error> {
    let uuid = uuid::Uuid::parse_str(uuid_str)?;
    Ok(uuid.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn account_id_deserialize() {
    let json_hex = json!("0x788b804aeea9afcf7042c6ee45ddc2a394f4e225918e3261a9b5ed5069037d09");
    let json_ss58 = json!("5Enm3VfZioxHVBpZgJcACv7pZPZZeYWymvrZ9cxkXhNHJWe5");
    let expected = AccountId::from_ss58check("5Enm3VfZioxHVBpZgJcACv7pZPZZeYWymvrZ9cxkXhNHJWe5")
      .expect("AccountId");

    let decoded: AccountId = serde_json::from_str(&json_hex.to_string()).expect("decode as json");
    assert_eq!(decoded, expected);
    let decoded: AccountId = serde_json::from_str(&json_ss58.to_string()).expect("decode as json");
    assert_eq!(decoded, expected);
  }

  #[test]
  fn account_id_roundtrip() {
    let account = AccountId::from_ss58check("5Enm3VfZioxHVBpZgJcACv7pZPZZeYWymvrZ9cxkXhNHJWe5")
      .expect("AccountId");
    let data = serde_json::to_string(&account).expect("encode json");
    let decoded: AccountId = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, account);
  }

  #[test]
  fn asset_id_roundtrip() {
    let asset: AssetId = "67e55044-10b1-426f-9247-bb680e5fe0c8"
      .parse()
      .expect("AssetId");
    let data = serde_json::to_string(&asset).expect("encode json");
    let decoded: AssetId = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, asset);
  }

  #[test]
  fn asset_id_display() {
    let str_id = "67e55044-10b1-426f-9247-bb680e5fe0c8";
    let asset: AssetId = str_id.parse().expect("AssetId");
    let display_id = format!("{asset}");
    assert_eq!(display_id, str_id);
  }

  #[test]
  fn multi_address_roundtrip() {
    let account = AccountId::from_ss58check("5Enm3VfZioxHVBpZgJcACv7pZPZZeYWymvrZ9cxkXhNHJWe5")
      .expect("AccountId");
    // Round-trip test MultiAddress::Id variant.
    let address = GenericAddress::Id(account);
    let data = serde_json::to_string(&address).expect("encode json");
    eprintln!("MultiAddress::Id = {data}");
    let decoded: GenericAddress = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, address);
    // Round-trip test MultiAddress::Index variant.
    let address = GenericAddress::Index(1234);
    let data = serde_json::to_string(&address).expect("encode json");
    eprintln!("MultiAddress::Index = {data}");
    let decoded: GenericAddress = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, address);
    // Round-trip test MultiAddress::Raw variant.
    let address = GenericAddress::Raw(vec![0, 1, 2, 3, 4, 5]);
    let data = serde_json::to_string(&address).expect("encode json");
    eprintln!("MultiAddress::Raw = {data}");
    let decoded: GenericAddress = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, address);
    // Round-trip test MultiAddress::Address32 variant.
    let address = GenericAddress::Address32([1; 32]);
    let data = serde_json::to_string(&address).expect("encode json");
    eprintln!("MultiAddress::Address32 = {data}");
    let decoded: GenericAddress = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, address);
    // Round-trip test MultiAddress::Address20 variant.
    let address = GenericAddress::Address20([2; 20]);
    let data = serde_json::to_string(&address).expect("encode json");
    eprintln!("MultiAddress::Address20 = {data}");
    let decoded: GenericAddress = serde_json::from_str(&data).expect("decode as json");
    assert_eq!(decoded, address);
  }
}

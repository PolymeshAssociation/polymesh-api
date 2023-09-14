use codec::{Compact, CompactAs, Decode, Encode};

use core::{fmt, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, de::DeserializeOwned, ser, Deserialize, Serialize};

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

#[cfg(feature = "serde")]
impl<AccountId, AccountIndex> ser::Serialize for MultiAddress<AccountId, AccountIndex>
where
  AccountId: AccountTraits,
  AccountIndex: AccountTraits,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    match self {
      Self::Id(account) => {
        if serializer.is_human_readable() {
          account.serialize(serializer)
        } else {
          serializer.serialize_newtype_variant("MultiAddress", 0, "Id", account)
        }
      }
      Self::Index(index) => serializer.serialize_newtype_variant("MultiAddress", 1, "Index", index),
      Self::Raw(data) => {
        if serializer.is_human_readable() {
          let h = hex::encode(data.as_slice());
          serializer.serialize_newtype_variant("MultiAddress", 2, "Raw", &h)
        } else {
          serializer.serialize_newtype_variant("MultiAddress", 2, "Raw", data)
        }
      }
      Self::Address32(address) => {
        if serializer.is_human_readable() {
          let h = hex::encode(&address[..]);
          serializer.serialize_newtype_variant("MultiAddress", 3, "Address32", &h)
        } else {
          serializer.serialize_newtype_variant("MultiAddress", 3, "Address32", address)
        }
      }
      Self::Address20(address) => {
        if serializer.is_human_readable() {
          let h = hex::encode(&address[..]);
          serializer.serialize_newtype_variant("MultiAddress", 4, "Address20", &h)
        } else {
          serializer.serialize_newtype_variant("MultiAddress", 4, "Address20", address)
        }
      }
    }
  }
}

#[cfg(feature = "serde")]
#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "PascalCase")]
enum MultiAddressField {
  Id,
  Index,
  Raw,
  Address32,
  Address20,
}

#[cfg(feature = "serde")]
#[derive(Default)]
struct MultiAddressVisitor<AccountId: AccountTraits, AccountIndex: AccountTraits> {
  is_human_readable: bool,
  _phantom: core::marker::PhantomData<(AccountId, AccountIndex)>,
}

#[cfg(feature = "serde")]
impl<AccountId: AccountTraits, AccountIndex: AccountTraits>
  MultiAddressVisitor<AccountId, AccountIndex>
{
  fn new(is_human_readable: bool) -> Self {
    Self {
      is_human_readable,
      _phantom: Default::default(),
    }
  }
}

#[cfg(feature = "serde")]
impl<'de, AccountId, AccountIndex> de::Visitor<'de> for MultiAddressVisitor<AccountId, AccountIndex>
where
  AccountId: AccountTraits,
  AccountIndex: AccountTraits,
{
  type Value = MultiAddress<AccountId, AccountIndex>;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("AccountId or MultiAddress")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let account = AccountId::from_str(v)
      .map_err(|_e| de::Error::custom(format!("Failed to decode AccountId")))?;
    Ok(MultiAddress::Id(account))
  }

  fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
  where
    V: de::MapAccess<'de>,
  {
    let mut address = None;
    while let Some(key) = map.next_key()? {
      match key {
        MultiAddressField::Id => {
          if address.is_some() {
            return Err(de::Error::duplicate_field("Id"));
          }
          address = Some(MultiAddress::Id(map.next_value()?));
        }
        MultiAddressField::Index => {
          if address.is_some() {
            return Err(de::Error::duplicate_field("Index"));
          }
          address = Some(MultiAddress::Index(map.next_value()?));
        }
        MultiAddressField::Raw => {
          if address.is_some() {
            return Err(de::Error::duplicate_field("Raw"));
          }
          if self.is_human_readable {
            let h: &str = map.next_value()?;
            let off = if h.starts_with("0x") { 2 } else { 0 };
            let data = hex::decode(&h[off..]).map_err(|e| de::Error::custom(e))?;
            address = Some(MultiAddress::Raw(data));
          } else {
            address = Some(MultiAddress::Raw(map.next_value()?));
          }
        }
        MultiAddressField::Address32 => {
          if address.is_some() {
            return Err(de::Error::duplicate_field("Address32"));
          }
          if self.is_human_readable {
            let h: &str = map.next_value()?;
            let mut data = [0u8; 32];
            let off = if h.starts_with("0x") { 2 } else { 0 };
            hex::decode_to_slice(&h[off..], &mut data).map_err(|e| de::Error::custom(e))?;
            address = Some(MultiAddress::Address32(data));
          } else {
            address = Some(MultiAddress::Address32(map.next_value()?));
          }
        }
        MultiAddressField::Address20 => {
          if address.is_some() {
            return Err(de::Error::duplicate_field("Address20"));
          }
          if self.is_human_readable {
            let h: &str = map.next_value()?;
            let mut data = [0u8; 20];
            let off = if h.starts_with("0x") { 2 } else { 0 };
            hex::decode_to_slice(&h[off..], &mut data).map_err(|e| de::Error::custom(e))?;
            address = Some(MultiAddress::Address20(data));
          } else {
            address = Some(MultiAddress::Address20(map.next_value()?));
          }
        }
      }
    }
    let address =
      address.ok_or_else(|| de::Error::missing_field("Id, Index, Raw, Address32, or Address20"))?;
    Ok(address)
  }
}

#[cfg(feature = "serde")]
impl<'de, AccountId, AccountIndex> Deserialize<'de> for MultiAddress<AccountId, AccountIndex>
where
  AccountId: AccountTraits,
  AccountIndex: AccountTraits,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    let visitor = MultiAddressVisitor::new(deserializer.is_human_readable());
    if deserializer.is_human_readable() {
      deserializer.deserialize_any(visitor)
    } else {
      deserializer.deserialize_enum(
        "MultiAddress",
        &["Id", "Index", "Raw", "Address32", "Address20"],
        visitor,
      )
    }
  }
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

#[cfg(feature = "serde")]
impl Serialize for AccountId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    if serializer.is_human_readable() {
      let ss58 = self.to_ss58check();
      serializer.serialize_str(&ss58)
    } else {
      self.0.serialize(serializer)
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for AccountId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    if deserializer.is_human_readable() {
      let h = Deserialize::deserialize(deserializer)?;
      Ok(AccountId::from_str(h).map_err(|e| de::Error::custom(e))?)
    } else {
      Ok(Self(Deserialize::deserialize(deserializer)?))
    }
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
    S: ser::Serializer,
  {
    if serializer.is_human_readable() {
      let h = hex::encode(&self.0);
      serializer.serialize_str(&h)
    } else {
      self.0.serialize(serializer)
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for IdentityId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    if deserializer.is_human_readable() {
      let h: &str = Deserialize::deserialize(deserializer)?;
      let mut id = IdentityId::default();
      let off = if h.starts_with("0x") { 2 } else { 0 };
      hex::decode_to_slice(&h[off..], &mut id.0).map_err(|e| de::Error::custom(e))?;
      Ok(id)
    } else {
      Ok(Self(Deserialize::deserialize(deserializer)?))
    }
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

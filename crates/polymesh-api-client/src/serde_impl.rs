use serde::{de, ser, Deserialize, Serialize};

use alloc::format;
use core::{fmt, str::FromStr};
use sp_core::crypto::Ss58Codec;

use crate::basic_types::{AccountId, AccountTraits, AssetId, IdentityId, MultiAddress};

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

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "PascalCase")]
enum MultiAddressField {
  Id,
  Index,
  Raw,
  Address32,
  Address20,
}

#[derive(Default)]
struct MultiAddressVisitor<AccountId: AccountTraits, AccountIndex: AccountTraits> {
  is_human_readable: bool,
  _phantom: core::marker::PhantomData<(AccountId, AccountIndex)>,
}

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

impl<'de> Deserialize<'de> for AccountId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    if deserializer.is_human_readable() {
      struct StringOrBytesVisitor;

      impl<'de> de::Visitor<'de> for StringOrBytesVisitor {
        type Value = AccountId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("a hex string or [u8; 32]")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
          E: de::Error,
        {
          Ok(AccountId::from_str(s).map_err(|e| de::Error::custom(e))?)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: de::SeqAccess<'de>,
        {
          let mut id = AccountId::default();
          for n in 0..32 {
            id.0[n] = seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(n, &self))?;
          }
          Ok(id)
        }
      }
      deserializer.deserialize_any(StringOrBytesVisitor)
    } else {
      Ok(Self(Deserialize::deserialize(deserializer)?))
    }
  }
}

impl Serialize for IdentityId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    if serializer.is_human_readable() {
      let h = format!("0x{}", hex::encode(&self.0));
      serializer.serialize_str(&h)
    } else {
      self.0.serialize(serializer)
    }
  }
}

impl<'de> Deserialize<'de> for IdentityId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    if deserializer.is_human_readable() {
      struct StringOrBytesVisitor;

      impl<'de> de::Visitor<'de> for StringOrBytesVisitor {
        type Value = IdentityId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("a hex string or [u8; 32]")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
          E: de::Error,
        {
          let mut id = IdentityId::default();
          let off = if s.starts_with("0x") { 2 } else { 0 };
          hex::decode_to_slice(&s[off..], &mut id.0).map_err(|e| de::Error::custom(e))?;
          Ok(id)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: de::SeqAccess<'de>,
        {
          let mut id = IdentityId::default();
          for n in 0..32 {
            id.0[n] = seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(n, &self))?;
          }
          Ok(id)
        }
      }
      deserializer.deserialize_any(StringOrBytesVisitor)
    } else {
      Ok(Self(Deserialize::deserialize(deserializer)?))
    }
  }
}

impl Serialize for AssetId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    self.as_uuid().serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for AssetId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    let uuid: uuid::Uuid = Deserialize::deserialize(deserializer)?;
    Ok(Self(uuid.into_bytes()))
  }
}

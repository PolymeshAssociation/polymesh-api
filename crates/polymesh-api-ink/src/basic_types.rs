use codec::{Decode, Encode};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use alloc::fmt;

#[cfg(feature = "std")]
use scale_info::TypeInfo;

pub use sp_arithmetic::per_things;

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Hash))]
#[cfg_attr(feature = "std", derive(TypeInfo))]
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

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct AccountId(pub [u8; 32]);

impl fmt::Debug for AccountId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
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

impl From<[u8; 32]> for AccountId {
  fn from(p: [u8; 32]) -> Self {
    Self(p)
  }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct IdentityId(pub [u8; 32]);

impl fmt::Debug for IdentityId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
  }
}

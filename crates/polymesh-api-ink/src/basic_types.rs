use codec::{Decode, Encode};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use alloc::fmt;

#[cfg(feature = "std")]
use scale_info::TypeInfo;

pub use sp_arithmetic::per_things;

pub use ink_env::AccountId;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

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

#[derive(
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Encode,
  Decode,
  SpreadAllocate,
  SpreadLayout,
  PackedLayout,
)]
#[cfg_attr(feature = "std", derive(TypeInfo, StorageLayout))]
pub struct IdentityId(pub [u8; 32]);

impl fmt::Debug for IdentityId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
  }
}

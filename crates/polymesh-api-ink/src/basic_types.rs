use codec::{CompactAs, Decode, Encode};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use alloc::fmt;

#[cfg(feature = "std")]
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

pub use ink::primitives::AccountId;
pub use sp_arithmetic::per_things;

pub use sp_core::{ecdsa, ed25519, sr25519};

// Re-impl `OldWeight`
#[derive(
  Clone, Copy, Debug, Encode, Decode, CompactAs, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct OldWeight(pub u64);

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum MultiSignature {
  /// An Ed25519 signature.
  Ed25519(ed25519::Signature),
  /// An Sr25519 signature.
  Sr25519(sr25519::Signature),
  /// An ECDSA/SECP256k1 signature.
  Ecdsa(ecdsa::Signature),
}

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
#[cfg_attr(feature = "std", derive(TypeInfo, StorageLayout))]
pub struct IdentityId(pub [u8; 32]);

impl fmt::Debug for IdentityId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = hex::encode(&self.0);
    write!(f, "0x{}", h)
  }
}

impl From<[u8; 32]> for IdentityId {
  fn from(raw: [u8; 32]) -> Self {
    Self(raw)
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "std", derive(TypeInfo, StorageLayout))]
pub struct AssetId([u8; 16]);

use ink::env::Environment;

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use scale_info::TypeInfo;

use alloc::vec::Vec;

use crate::{Encoded, IdentityId, AccountId};

#[ink::chain_extension]
#[derive(Clone, Copy)]
pub trait PolymeshRuntime {
  type ErrorCode = PolymeshRuntimeErr;

  #[ink(extension = 0x00_00_00_01)]
  fn call_runtime(call: Encoded);

  #[ink(extension = 0x00_00_00_02)]
  fn read_storage(key: Encoded) -> Option<Vec<u8>>;

  #[ink(extension = 0x00_00_00_03)]
  fn get_spec_version() -> u32;

  #[ink(extension = 0x00_00_00_04)]
  fn get_transaction_version() -> u32;

  #[ink(extension = 0x00_00_00_05)]
  fn get_key_did(key: AccountId) -> Option<IdentityId>;

  #[ink(extension = 0x00_00_00_10)]
  fn twox_64(data: Encoded) -> [u8; 8];

  #[ink(extension = 0x00_00_00_11)]
  fn twox_128(data: Encoded) -> [u8; 16];

  #[ink(extension = 0x00_00_00_12)]
  fn twox_256(data: Encoded) -> [u8; 32];

  #[ink(extension = 0x00_00_00_13)]
  fn get_latest_api_upgrade(api: Encoded) -> [u8; 32];
}

pub type PolymeshRuntimeInstance = <PolymeshRuntime as ink::ChainExtensionInstance>::Instance;

pub fn new_instance() -> PolymeshRuntimeInstance {
  <PolymeshRuntime as ink::ChainExtensionInstance>::instantiate()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum PolymeshRuntimeErr {
  Unknown,
}

impl ink::env::chain_extension::FromStatusCode for PolymeshRuntimeErr {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      1 => Err(Self::Unknown),
      _ => panic!("encountered unknown status code"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolymeshEnvironment {}

impl Environment for PolymeshEnvironment {
  const MAX_EVENT_TOPICS: usize = <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

  type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
  type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
  type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
  type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
  type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

  type ChainExtension = PolymeshRuntime;
}

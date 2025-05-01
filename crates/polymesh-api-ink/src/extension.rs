use ink::env::Environment;

use codec::{Decode, Encode};

use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::{AccountId, AssetId, Encoded, Error, IdentityId};

#[ink::chain_extension]
#[derive(Clone, Copy)]
pub trait PolymeshRuntime {
  type ErrorCode = Error;

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

  #[ink(extension = 0x00_00_00_14)]
  fn call_runtime_with_error(call: Encoded) -> Result<Result<(), CallRuntimeError>, Error>;

  #[ink(extension = 0x00_00_00_15)]
  fn get_next_asset_id(account_id: AccountId) -> AssetId;
}

pub type PolymeshRuntimeInstance = <PolymeshRuntime as ink::ChainExtensionInstance>::Instance;

pub fn new_instance() -> PolymeshRuntimeInstance {
  <PolymeshRuntime as ink::ChainExtensionInstance>::instantiate()
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct CallRuntimeError(pub String);

impl From<CallRuntimeError> for Error {
  fn from(err: CallRuntimeError) -> Self {
    Self::ExtrinsicCallFailed { error_msg: err.0 }
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

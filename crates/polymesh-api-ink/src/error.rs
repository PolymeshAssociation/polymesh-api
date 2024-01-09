use alloc::{format, string::String};

use codec::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
  ParityScaleCodec(String),
  Generic { status_code: u32 },
  ExtrinsicCallFailed { error_msg: String },
}

impl From<codec::Error> for Error {
  fn from(err: codec::Error) -> Self {
    Self::ParityScaleCodec(format!("{err:?}"))
  }
}

impl ink::env::chain_extension::FromStatusCode for Error {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      _ => Err(Self::Generic { status_code }),
    }
  }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

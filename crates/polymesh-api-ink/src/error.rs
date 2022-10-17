use alloc::{format, string::String};

use crate::extension::PolymeshRuntimeErr;

use codec::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
  ParityScaleCodec(String),
  RuntimeError(PolymeshRuntimeErr),
}

impl From<PolymeshRuntimeErr> for Error {
  fn from(err: PolymeshRuntimeErr) -> Self {
    Self::RuntimeError(err)
  }
}

impl From<codec::Error> for Error {
  fn from(err: codec::Error) -> Self {
    Self::ParityScaleCodec(format!("{err:?}"))
  }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

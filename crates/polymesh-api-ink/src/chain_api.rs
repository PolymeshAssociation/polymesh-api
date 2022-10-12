use alloc::{fmt, vec::Vec};

use codec::{Decode, Encode};

use crate::*;

pub trait RuntimeTraits: Clone + Encode + Decode + fmt::Debug {}

impl<T> RuntimeTraits for T where T: Clone + Encode + Decode + fmt::Debug {}

/*
#[derive(Clone, Debug)]
pub enum ExtrinsicResult<Api: ChainApi + ?Sized> {
  Success(Api::DispatchInfo),
  Failed(Api::DispatchInfo, Api::DispatchError),
}

impl<Api: ChainApi> ExtrinsicResult<Api> {
  pub fn is_success(&self) -> bool {
    match self {
      Self::Success(_) => true,
      Self::Failed(_, _) => false,
    }
  }

  pub fn is_failed(&self) -> bool {
    match self {
      Self::Success(_) => false,
      Self::Failed(_, _) => true,
    }
  }

  pub fn ok(&self) -> Result<()> {
    match self {
      Self::Success(_) => Ok(()),
      Self::Failed(_, err) => Err(Error::ExtrinsicError(format!("{:?}", err))),
    }
  }
}
*/

pub trait ChainApi {
  type RuntimeCall: RuntimeTraits;
  type RuntimeEvent: RuntimeTraits;
  type DispatchInfo: RuntimeTraits;
  type DispatchError: RuntimeTraits;
}

pub struct Call {
  call: Vec<u8>,
}

impl Call {
  pub fn new(call: Vec<u8>) -> Self {
    Self { call }
  }

  pub fn encoded(&self) -> Encoded {
    Encoded(self.call.clone())
  }

  pub fn submit(&self) -> Result<()> {
    let runtime = polymesh_extension::new_instance();
    Ok(runtime.call_runtime(self.into())?)
  }
}

impl Encode for Call {
  fn size_hint(&self) -> usize {
    self.call.len()
  }
  fn encode_to<T: ::codec::Output + ?Sized>(&self, dest: &mut T) {
    dest.write(&self.call);
  }
}

impl fmt::Debug for Call {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.call.fmt(f)
  }
}

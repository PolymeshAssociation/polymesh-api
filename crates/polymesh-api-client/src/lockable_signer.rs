use core::ops::{Deref, DerefMut};
use std::sync::Arc;

use sp_runtime::MultiSignature;
use sp_std::prelude::*;

use tokio::sync::{Mutex, OwnedMutexGuard};

use async_trait::async_trait;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

use crate::*;

#[derive(Clone)]
pub struct LockableSigner<S>(Arc<Mutex<S>>);

impl<S: Signer + 'static> LockableSigner<S> {
  pub fn new(signer: S) -> Self {
    Self(Arc::new(Mutex::new(signer)))
  }

  pub async fn lock(&self) -> LockedSigner<S> {
    LockedSigner(self.0.clone().lock_owned().await)
  }
}

pub struct LockedSigner<S>(OwnedMutexGuard<S>);

impl<S> Deref for LockedSigner<S> {
  type Target = S;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S> DerefMut for LockedSigner<S> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[async_trait]
impl<S: Signer> Signer for LockedSigner<S> {
  fn account(&self) -> AccountId {
    self.0.account()
  }

  async fn nonce(&self) -> Option<u32> {
    self.0.nonce().await
  }

  async fn set_nonce(&mut self, nonce: u32) {
    self.0.set_nonce(nonce).await
  }

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature> {
    self.0.sign(msg).await
  }
}

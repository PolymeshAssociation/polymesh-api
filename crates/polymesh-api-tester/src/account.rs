use std::sync::Arc;

use sp_keyring::{ed25519, sr25519};
use sp_runtime::MultiSignature;

use polymesh_api::client::{
  AccountId, KeypairSigner, LockableSigner, LockedSigner, PairSigner, Signer,
};

use crate::error::Result;
use crate::Db;

struct AccountSignerInner {
  signer: Box<dyn Signer + Send + Sync>,
  db: Option<Db>,
}

#[async_trait::async_trait]
impl Signer for AccountSignerInner {
  fn account(&self) -> AccountId {
    self.signer.account()
  }

  async fn nonce(&self) -> Option<u32> {
    match &self.db {
      Some(db) => db.get_nonce(self.account()).await.ok(),
      None => None,
    }
  }

  async fn set_nonce(&mut self, nonce: u32) {
    match &self.db {
      Some(db) => {
        if let Err(err) = db.set_nonce(self.account(), nonce).await {
          log::error!("Failed to update account nonce in DB: {err:?}");
        }
      }
      None => (),
    }
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<MultiSignature> {
    Ok(self.signer.sign(msg).await?)
  }
}

/// AccountSigner is wrapper for signing keys (sr25519, ed25519, etc...).
#[derive(Clone)]
pub struct AccountSigner {
  signer: Arc<LockableSigner>,
  account: AccountId,
}

impl AccountSigner {
  pub fn new<P: KeypairSigner + 'static>(db: Option<Db>, pair: P) -> Self {
    let signer = PairSigner::new(pair);
    let account = signer.account();
    Self {
      signer: Arc::new(LockableSigner::new(AccountSignerInner {
        signer: Box::new(signer),
        db: db.clone(),
      })),
      account,
    }
  }

  pub fn alice(db: Option<Db>) -> Self {
    Self::new(db, sr25519::Keyring::Alice.pair())
  }

  pub fn bob(db: Option<Db>) -> Self {
    Self::new(db, sr25519::Keyring::Bob.pair())
  }

  /// Generate signing key pair from string `s`.
  pub fn from_string(db: Option<Db>, s: &str) -> Result<Self> {
    Ok(Self::new(
      db,
      <sr25519::sr25519::Pair as KeypairSigner>::from_string(s, None)?,
    ))
  }
}

impl From<sr25519::Keyring> for AccountSigner {
  fn from(key: sr25519::Keyring) -> Self {
    Self::new(None, key.pair())
  }
}

impl From<ed25519::Keyring> for AccountSigner {
  fn from(key: ed25519::Keyring) -> Self {
    Self::new(None, key.pair())
  }
}

#[async_trait::async_trait]
impl Signer for AccountSigner {
  fn account(&self) -> AccountId {
    self.account.clone()
  }

  async fn nonce(&self) -> Option<u32> {
    let inner = self.signer.lock().await;
    inner.nonce().await
  }

  async fn set_nonce(&mut self, nonce: u32) {
    let mut inner = self.signer.lock().await;
    inner.set_nonce(nonce).await
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<MultiSignature> {
    let inner = self.signer.lock().await;
    Ok(inner.sign(msg).await?)
  }

  async fn lock(&self) -> Option<LockedSigner<'_>> {
    Some(self.signer.lock().await)
  }
}

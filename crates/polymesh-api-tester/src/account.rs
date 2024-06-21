use std::sync::Arc;

use sp_keyring::{ed25519, sr25519};
use sp_runtime::MultiSignature;

use polymesh_api::client::{AccountId, KeypairSigner, LockableSigner, PairSigner, Signer};

use crate::error::Result;
use crate::Db;

/// DbAccountSigner is wrapper for signing keys (sr25519, ed25519, etc...) and using
/// a local Database for managing account nonces.
#[derive(Clone)]
pub struct DbAccountSigner {
  signer: Arc<dyn Signer + Send + Sync>,
  db: Db,
  account: AccountId,
}

impl DbAccountSigner {
  pub fn new<P: KeypairSigner + 'static>(db: Db, pair: P) -> Self {
    let signer = PairSigner::new(pair);
    let account = signer.account();
    Self {
      signer: Arc::new(signer),
      db: db.clone(),
      account,
    }
  }

  pub fn alice(db: Db) -> Self {
    Self::new(db, sr25519::Keyring::Alice.pair())
  }

  pub fn bob(db: Db) -> Self {
    Self::new(db, sr25519::Keyring::Bob.pair())
  }

  /// Generate signing key pair from string `s`.
  pub fn from_string(db: Db, s: &str) -> Result<Self> {
    Ok(Self::new(
      db,
      <sr25519::sr25519::Pair as KeypairSigner>::from_string(s, None)?,
    ))
  }
}

#[async_trait::async_trait]
impl Signer for DbAccountSigner {
  fn account(&self) -> AccountId {
    self.account
  }

  async fn nonce(&self) -> Option<u32> {
    self.db.get_next_nonce(self.account).await.ok()
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<MultiSignature> {
    Ok(self.signer.sign(msg).await?)
  }
}

/// AccountSigner is wrapper for signing keys (sr25519, ed25519, etc...).
#[derive(Clone)]
pub struct AccountSigner {
  signer: Arc<LockableSigner<Box<dyn Signer>>>,
  account: AccountId,
}

impl AccountSigner {
  pub fn new<P: KeypairSigner + 'static>(pair: P) -> Self {
    let signer = PairSigner::new(pair);
    let account = signer.account();
    Self {
      signer: Arc::new(LockableSigner::new(Box::new(signer))),
      account,
    }
  }

  /// Generate signing key pair from string `s`.
  pub fn from_string(s: &str) -> Result<Self> {
    Ok(Self::new(
      <sr25519::sr25519::Pair as KeypairSigner>::from_string(s, None)?,
    ))
  }
}

impl From<sr25519::Keyring> for AccountSigner {
  fn from(key: sr25519::Keyring) -> Self {
    Self::new(key.pair())
  }
}

impl From<ed25519::Keyring> for AccountSigner {
  fn from(key: ed25519::Keyring) -> Self {
    Self::new(key.pair())
  }
}

#[async_trait::async_trait]
impl Signer for AccountSigner {
  fn account(&self) -> AccountId {
    self.account.clone()
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<MultiSignature> {
    let locked = self.signer.lock().await;
    locked.sign(msg).await
  }

  async fn lock(&self) -> Option<Box<dyn Signer>> {
    let locked = self.signer.lock().await;
    Some(Box::new(locked))
  }
}

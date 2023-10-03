use std::sync::Arc;

use sp_keyring::{ed25519, sr25519};
use sp_runtime::MultiSignature;

use polymesh_api::client::{AccountId, KeypairSigner, PairSigner, Signer};

use crate::error::Result;
use crate::Db;
use crate::User;

/// AccountSigner is wrapper for signing keys (sr25519, ed25519, etc...).
#[derive(Clone)]
pub struct AccountSigner {
  signer: Arc<dyn Signer + Send + Sync>,
  account: AccountId,
  db: Option<Db>,
}

impl AccountSigner {
  pub fn new<P: KeypairSigner + 'static>(db: Option<Db>, pair: P) -> Self {
    let signer = PairSigner::new(pair);
    let account = signer.account();
    Self {
      signer: Arc::new(signer),
      account,
      db,
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

impl From<AccountSigner> for User {
  fn from(signer: AccountSigner) -> User {
    User { signer, did: None }
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
    match &self.db {
      Some(db) => db.get_nonce(self.account).await.ok(),
      None => None,
    }
  }

  async fn set_nonce(&mut self, _nonce: u32) {}

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<MultiSignature> {
    Ok(self.signer.sign(msg).await?)
  }
}

pub use polymesh_api::{
  client::{AccountId, AssetId, IdentityId, Signer},
  polymesh::types::{
    polymesh_primitives::{
      secondary_key::{KeyRecord, Permissions, SecondaryKey},
      ticker::Ticker,
    },
    runtime::{events::*, RuntimeEvent},
  },
  Api,
};
pub use polymesh_api_client_extras as extras;

mod error;
use error::*;

mod db;
pub use db::*;

mod account;
pub use account::*;

mod tester;
pub use tester::*;

pub async fn client_api() -> Result<Api> {
  let url = std::env::var("POLYMESH_URL").unwrap_or_else(|_| "ws://localhost:9944".into());
  Ok(Api::new(&url).await?)
}

#[derive(Clone)]
pub struct User {
  pub api: Api,
  /// Primary key signer.
  pub primary_key: AccountSigner,
  /// User's secondary keys.
  pub secondary_keys: Vec<AccountSigner>,
  /// User's identity if they have been onboarded.
  pub did: Option<IdentityId>,
}

#[async_trait::async_trait]
impl Signer for User {
  fn account(&self) -> AccountId {
    self.primary_key.account()
  }

  async fn nonce(&self) -> Option<u32> {
    self.primary_key.nonce().await
  }

  async fn set_nonce(&mut self, nonce: u32) {
    self.primary_key.set_nonce(nonce).await
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<sp_runtime::MultiSignature> {
    Ok(self.primary_key.sign(msg).await?)
  }

  async fn lock(&self) -> Option<Box<dyn Signer>> {
    self.primary_key.lock().await
  }
}

impl User {
  pub fn new(api: &Api, primary_key: AccountSigner) -> Self {
    Self {
      api: api.clone(),
      primary_key,
      secondary_keys: Vec::new(),
      did: None,
    }
  }

  pub fn account(&self) -> AccountId {
    self.primary_key.account()
  }
}

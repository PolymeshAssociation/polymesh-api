use polymesh_api::client::Signer;
pub use polymesh_api::{
  client::{AccountId, IdentityId},
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
  pub signer: AccountSigner,
  pub did: Option<IdentityId>,
}

#[async_trait::async_trait]
impl Signer for User {
  fn account(&self) -> AccountId {
    self.signer.account()
  }

  async fn nonce(&self) -> Option<u32> {
    self.signer.nonce().await
  }

  async fn set_nonce(&mut self, nonce: u32) {
    self.signer.set_nonce(nonce).await
  }

  async fn sign(&self, msg: &[u8]) -> polymesh_api::client::Result<sp_runtime::MultiSignature> {
    Ok(self.signer.sign(msg).await?)
  }
}

impl User {
  pub fn new(signer: AccountSigner) -> Self {
    Self { signer, did: None }
  }

  pub fn account(&self) -> AccountId {
    self.signer.account()
  }
}

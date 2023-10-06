use std::env;
use std::str::FromStr;

use futures_util::StreamExt;

use anyhow::Result;

use sp_keyring::AccountKeyring;

use polymesh_api::client::{AccountId, IdentityId};
use polymesh_api::polymesh::types::{
  polymesh_primitives::secondary_key::Signatory,
};
use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let signatory = match env::args().nth(2) {
    Some(s) if s.starts_with("0x") => {
      let mut id = [0u8; 32];
      hex::decode_to_slice(&s[2..], &mut id[..])?;
      Signatory::Identity(IdentityId(id))
    },
    Some(s) => Signatory::Account(AccountId::from_str(&s)?),
    None => Signatory::Account(AccountKeyring::Bob.to_account_id().into()),
  };

  let api = Api::new(&url).await?;

  // Query only the `AuthorizationId` from the storage key.
  let auths = api
    .paged_query()
    .identity()
    .authorizations(signatory.clone())
    .keys();
  tokio::pin!(auths);
  println!("paged_query.auths.keys:");
  while let Some(auth_id) = auths.next().await {
    println!(" -- {}", auth_id?);
  }

  // Query only the `Authorization` values.
  let auths = api
    .paged_query()
    .identity()
    .authorizations(signatory.clone())
    .values();
  tokio::pin!(auths);
  println!("paged_query.auths.values:");
  while let Some(auth) = auths.next().await {
    println!(" -- {:?}", auth?);
  }

  // Query both the `AuthorizationId` keys and `Authorization` values.
  let auths = api
    .paged_query()
    .identity()
    .authorizations(signatory)
    .entries();
  tokio::pin!(auths);
  println!("paged_query.auths.entries:");
  while let Some(auth) = auths.next().await {
    let (id, auth) = auth?;
    println!(" -- [{id}] = {auth:?}");
  }

  Ok(())
}

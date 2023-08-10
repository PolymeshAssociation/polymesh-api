use std::env;

use anyhow::{bail, Result};

use sp_runtime::MultiSignature;

use codec::Encode;

use polymesh_api::client::{DefaultSigner, Signer};
use polymesh_api::polymesh::types::{
  polymesh_common_utilities::traits::identity::SecondaryKeyWithAuth,
  polymesh_primitives::secondary_key::{KeyRecord, Permissions, SecondaryKey},
  primitive_types::H512,
};
use polymesh_api::Api;
use polymesh_api_client_extras::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let mut args = env::args();
  args.next(); // Skip program name.
  let url = args.next().expect("Missing ws url");
  let p_key = args.next().expect("Missing primary key");

  let mut primary_key = DefaultSigner::from_string(&p_key, None)?;

  let api = Api::new(&url).await?;
  let identity = api.query().identity();

  // Get Alice's Identity and offchain authorization nonce.
  let target_id = match identity.key_records(primary_key.account).await? {
    Some(KeyRecord::PrimaryKey(did)) => did,
    Some(_) => {
      panic!("Must use primary key to add secondary keys.");
    }
    None => {
      panic!("{:?} doesn't have an identity.", primary_key.account);
    }
  };
  let nonce = identity.off_chain_authorization_nonce(target_id).await?;

  // Get current timestamp from chain.
  let now = api.query().timestamp().now().await?;
  let expires_at = now + 60_000; // Expire after 1 minute (ms).

  // Prepare authorazation data.
  let auth = TargetIdAuthorization {
    target_id,
    nonce,
    expires_at,
  };
  let auth_data = auth.encode();

  // Secondary keys with authorization and permissions.
  let permissions: Permissions = serde_json::from_value(serde_json::json!({
    "asset": { "These": [ b"ABC123456789" ]},
    "extrinsic": "Whole",
    "portfolio": "Whole",
  }))?;
  let mut keys = Vec::new();
  for key in args {
    let key = DefaultSigner::from_string(&key, None)?;
    match key.sign(&auth_data[..]).await? {
      MultiSignature::Sr25519(sig) => {
        keys.push(SecondaryKeyWithAuth {
          secondary_key: SecondaryKey {
            key: key.account,
            permissions: permissions.clone(),
          },
          auth_signature: H512(sig.0),
        });
      }
      _ => {
        bail!("Only Sr25519 keys supported.");
      }
    }
  }
  if keys.len() == 0 {
    bail!("Need at least one more key seed.");
  }

  let mut res = api
    .call()
    .identity()
    .add_secondary_keys_with_authorization(keys, expires_at)?
    .sign_submit_and_watch(&mut primary_key)
    .await?;
  let events = res.events().await?;
  println!("Add secondary keys with auth: events = {:#?}", events);
  Ok(())
}

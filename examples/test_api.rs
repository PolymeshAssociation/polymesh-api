use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;
use codec::Encode;

use sub_api::rpc::*;

use sub_api_macro::*;

#[sub_api(metadata_file = "specs/polymesh_dev_spec_5000001.meta")]
pub mod polymesh {}
use polymesh::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = RpcClient::new(&url).await?;

  // Get current Metadata.
  let metadata = client.get_metadata(None).await?;

  let api = Api::from(metadata);

  let dest_id = AccountKeyring::Bob.to_account_id();
  let dest = &dest_id;
  let call = api.call().balances().transfer(dest.into(), 123_012_345)?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!("call_json = {:#?}", serde_json::to_string(&call));

  // Test batches.
  let call = api.call().utility().batch(vec![
    api.call().balances().transfer(dest.into(), 1)?,
    api.call().balances().transfer(dest.into(), 2)?,
    api.call().balances().transfer(dest.into(), 3)?,
  ])?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!("call_json = {:#?}", serde_json::to_string(&call));

  Ok(())
}

use std::env;

use anyhow::Result;

use sp_runtime::MultiAddress;
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

  let dest = MultiAddress::from(AccountKeyring::Bob.to_account_id());
  let call = api.call().balances().transfer(dest.clone(), 123_012_345);
  println!("balances.transfer = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));

  // Test batches.
  let batch_calls = api.call().utility().batch(vec![
    api.call().balances().transfer(dest.clone(), 1),
    api.call().balances().transfer(dest.clone(), 2),
    api.call().balances().transfer(dest, 3),
  ]);
  println!("encoded = {}", hex::encode(batch_calls.encode()));

  Ok(())
}

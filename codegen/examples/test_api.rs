use std::env;

use anyhow::Result;

use sp_runtime::MultiAddress;
use sp_keyring::AccountKeyring;

use sub_api::rpc::*;

use codegen::*;

pub mod polymesh;
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
  Ok(())
}

//use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;

//use sub_api::rpc::*;

use sub_api_macro::*;

#[sub_api(metadata_file = "specs/polymesh_dev_spec_5000001.meta")]
pub mod polymesh {}
use polymesh::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  //let url = env::args().nth(1).expect("Missing ws url");

  //let _client = RpcClient::new(&url).await?;

  let dest = AccountKeyring::Bob.to_account_id().into();
  let api = Api::new();
  let call = api.call.balances.transfer(dest, 123_012_345.into());
  println!("balances.transfer = {call:#?}");
  Ok(())
}

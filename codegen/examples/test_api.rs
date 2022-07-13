use std::env;

use anyhow::Result;

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

  //let version = client.get_runtime_version(None).await?;
  //println!("{version:#?}");

  let code = generate(metadata)?;
  println!("{}", rustfmt_wrapper::rustfmt(code).unwrap());
  //println!("{code}");

  let dest = AccountKeyring::Bob.to_account_id().into();
  let api = Api::new();
  let call = api.call.balances.transfer(dest, 123_012_345);
  println!("balances.transfer = {call:#?}");
  Ok(())
}

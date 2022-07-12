use std::env;

use anyhow::Result;

use sub_api::rpc::*;

use codegen::*;

mod polymesh_api;
use polymesh_api::*;

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

  Ok(())
}

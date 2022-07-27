use std::env;

use anyhow::Result;

use sub_api::rpc::*;
use sub_api::schema::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = RpcClient::new(&url).await?;

  // Get block hash
  let gen_hash = client.get_block_hash(0).await?;
  println!("gen_hash = {gen_hash}");

  let version = client.get_runtime_version(None).await?;
  println!("{version:#?}");

  // Get current Metadata.
  let metadata = client.get_metadata(None).await?;
  println!("metadata = {metadata:#?}");

  let mut types = Types::new();
  types.load_schema("./schemas/init_types.json")?;
  types.load_schema("./schemas/polymesh/5000001.json")?;

  //types.dump_types();
  types.dump_unresolved();
  Ok(())
}

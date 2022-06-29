use anyhow::Result;

use sub_codegen_api::schema::*;
use sub_codegen_api::rpc::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = "ws://comp002:9944/";

  let client = RpcClient::new(&url).await?;

  // Get block hash
  let gen_hash = client.get_block_hash(0).await?;
  println!("gen_hash = {gen_hash}");

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

use std::env;
use std::str::FromStr;

use anyhow::Result;

use polymesh_api_client::schema::*;
use polymesh_api_client::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = Client::new(&url).await?;
  let types_registry = TypesRegistry::new("./schemas/init_types.json".into(), "schema.json".into());

  // Get block hash
  let gen_hash = client.get_block_hash(0).await?;
  println!("gen_hash = {gen_hash:?}");

  //let hash = Some(BlockHash::from_str("0x921e3fec73cfa1ad468da31eaa5ea012ecc32bf1a68acd254ea804efa248bd7f")?);
  let hash = None;
  // Get current block runtime version.
  let version = client.get_block_runtime_version(hash).await?.unwrap();
  println!("spec: {} - {}", version.spec_name, version.spec_version);

  let types = types_registry.get_block_types(&client, Some(version), hash).await?;

  types.dump_types();
  types.dump_unresolved();
  Ok(())
}

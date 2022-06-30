use std::fs::File;
use std::io::prelude::*;
use std::env;

use anyhow::Result;

use sub_api::rpc::*;

use codec::Encode;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = RpcClient::new(&url).await?;

  let rt = client.get_runtime_version(None).await?;
  println!("Download chain metadata for Runtime: {}, spec: {}", rt.spec_name, rt.spec_version);
  let filename = format!("{}_spec_{}.meta", rt.spec_name, rt.spec_version);

  // Get current Metadata.
  let metadata = client.get_metadata(None).await?;

  let raw_metadata = metadata.encode();

  let mut file = File::create(filename.clone())?;
  file.write_all(&raw_metadata)?;
  println!("Wrote file: {}", filename);

  Ok(())
}

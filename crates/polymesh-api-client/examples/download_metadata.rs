use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use anyhow::Result;

use polymesh_api_client::*;

use codec::Encode;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let block_hash = env::args()
    .nth(2)
    .and_then(|ref h| BlockHash::from_str(h).ok());

  let client = Client::new(&url).await?;

  let rt = client.get_block_runtime_version(block_hash).await?.expect("RuntimeVersion");
  println!(
    "Download chain metadata for Runtime: {}, spec: {}",
    rt.spec_name, rt.spec_version
  );
  let filename = format!("{}_spec_{}.meta", rt.spec_name, rt.spec_version);

  // Get current Metadata.
  let metadata = client.get_block_metadata(block_hash).await?;

  let raw_metadata = metadata.encode();

  let mut file = File::create(filename.clone())?;
  file.write_all(&raw_metadata)?;
  println!("Saved metadata to file: {}", filename);

  Ok(())
}

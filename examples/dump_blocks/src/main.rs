use std::env;

use anyhow::Result;

use polymesh_api::Api;
use polymesh_api::ChainApi;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let start_block = env::args()
    .nth(2)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 0);
  let count = env::args()
    .nth(3)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 10);
  let end_block = start_block + count;
  let skip = env::args()
    .nth(4)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 1);

  let api = Api::new(&url).await?;
  let client = api.client();

  let mut block_number = start_block;
  while block_number < end_block {
    let hash = client.get_block_hash(block_number).await?;
    let events = api.block_events(hash).await?;
    if events.len() > 1 {
      println!("block[{block_number}] events: {}", serde_json::to_string_pretty(&events)?);
    }
    block_number += skip;
  }

  Ok(())
}

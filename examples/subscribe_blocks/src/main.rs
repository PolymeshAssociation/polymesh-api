use std::env;

use anyhow::Result;

use polymesh_api::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let api = Api::new(&url).await?;
  let client = api.client();

  let mut sub_blocks = client.subscribe_blocks().await?;

  while let Some(header) = sub_blocks.next().await.transpose()? {
    println!("{}: {}", header.number, header.hash());
  }

  Ok(())
}

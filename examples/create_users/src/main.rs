use std::env;

use anyhow::Result;

use polymesh_api_client_extras::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let prefix = env::args().nth(2).expect("User prefix");
  let count = env::args()
    .nth(3)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 10);

  let mut helper = PolymeshHelper::new(&url).await?;

  let users = helper.generate_prefix_users(&prefix, count).await?;
  eprintln!("Generated {}", users.len());
  Ok(())
}

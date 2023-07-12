use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;

use polymesh_api::client::PairSigner;
use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let mut alice = PairSigner::new(AccountKeyring::Alice.pair());

  let api = Api::new(&url).await?;

  let dest = AccountKeyring::Bob.to_account_id().into();
  let mut res = api
    .call()
    .balances()
    .transfer(dest, 123_012_345)?
    .execute(&mut alice)
    .await?;
  let events = res.events().await?;
  println!("call1 events = {:#?}", events);
  Ok(())
}

use std::env;

use anyhow::Result;

use codec::Encode;
use sp_keyring::AccountKeyring;

use polymesh_api_client::PairSigner;

use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let mut alice = PairSigner::new(AccountKeyring::Alice.pair());

  let api = Api::new(&url).await?;

  let dest_id = AccountKeyring::Bob.to_account_id();
  let dest = &dest_id;
  let call = api.call().balances().transfer(dest.into(), 123_012_345)?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!(
    "call_json = {:#?}",
    serde_json::to_string(call.runtime_call())?
  );
  let mut res1 = call.sign_submit_and_watch(&mut alice).await?;

  // Test batches.
  let call = api.call().utility().batch(vec![
    api.call().balances().transfer(dest.into(), 1)?.into(),
    api.call().balances().transfer(dest.into(), 2)?.into(),
    api.call().balances().transfer(dest.into(), 3)?.into(),
  ])?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!(
    "call_json = {:#?}",
    serde_json::to_string(call.runtime_call())?
  );
  let mut res2 = call.sign_submit_and_watch(&mut alice).await?;
  println!("call1 result = {:?}", res1.wait_in_block().await);
  println!("call2 result = {:?}", res2.wait_in_block().await);

  println!("call1 events = {:#?}", res1.events().await);
  println!("call2 events = {:#?}", res2.events().await);
  Ok(())
}

use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;

use polymesh_api::Api;
use polymesh_api::client::MultiAddress;
use polymesh_api::client::PairSigner;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let mut alice = PairSigner::new(AccountKeyring::Alice.pair());

  let api = Api::new(&url).await?;

  let dest: MultiAddress<_, _> = AccountKeyring::Bob.to_account_id().into();
  let call = api.call().balances().transfer(dest.clone(), 123_012_345)?;
  println!("call = {call:#?}");
  println!(
    "call_json = {:#?}",
    serde_json::to_string(call.runtime_call())?
  );
  let mut res1 = call.sign_submit_and_watch(&mut alice).await?;

  // Test batches.
  let call = api.call().utility().batch(vec![
    api.call().balances().transfer(dest.clone(), 1)?.into(),
    api.call().balances().transfer(dest.clone(), 2)?.into(),
    api.call().balances().transfer(dest.clone(), 3)?.into(),
  ])?;
  println!("call = {call:#?}");
  println!(
    "call_json = {:#?}",
    serde_json::to_string(call.runtime_call())?
  );
  let mut res2 = call.sign_submit_and_watch(&mut alice).await?;
  println!("call1 result = {:?}", res1.wait_in_block().await);
  println!("call2 result = {:?}", res2.wait_in_block().await);

  let events = res1.events().await?;
  println!("call1 events = {:#?}", serde_json::to_string(&events));
  let events = res2.events().await?;
  println!("call2 events = {:#?}", serde_json::to_string(&events));
  Ok(())
}

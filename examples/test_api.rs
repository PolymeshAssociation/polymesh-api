use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;
use codec::Encode;

use sub_api::SimpleSigner;

use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let mut alice = SimpleSigner::new(AccountKeyring::Alice.pair());

  let api = Api::new(&url).await?;

  let dest_id = AccountKeyring::Bob.to_account_id();
  let dest = &dest_id;
  let call = api.call().balances().transfer(dest.into(), 123_012_345)?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!("call_json = {:#?}", serde_json::to_string(call.runtime_call()));
  let mut res1 = alice.submit_and_watch(&call).await?;

  // Test batches.
  let call = api.call().utility().batch(vec![
    api.call().balances().transfer(dest.into(), 1)?.into(),
    api.call().balances().transfer(dest.into(), 2)?.into(),
    api.call().balances().transfer(dest.into(), 3)?.into(),
  ])?;
  println!("call = {call:#?}");
  println!("encoded = {}", hex::encode(call.encode()));
  println!("call_json = {:#?}", serde_json::to_string(call.runtime_call()));
  let mut res2 = alice.submit_and_watch(&call).await?;
  println!("call1 result = {:?}", res1.next().await);
  println!("call2 result = {:?}", res2.next().await);

  Ok(())
}

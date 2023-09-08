use std::env;

use anyhow::Result;

use sp_keyring::AccountKeyring;

use polymesh_api::client::PairSigner;
use polymesh_api::polymesh::types::runtime::{events, RuntimeEvent};
use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  //let mut alice = PairSigner::new(AccountKeyring::Alice.pair());
  let mut alice = PairSigner::new(subxt_signer::sr25519::dev::alice());

  let api = Api::new(&url).await?;

  let dest = AccountKeyring::Bob.to_account_id().into();
  let mut res = api
    .call()
    .balances()
    .transfer(dest, 123_012_345)?
    .execute(&mut alice)
    .await?;
  let events = res.events().await?;
  //println!("call1 events = {:#?}", events);
  if let Some(events) = events {
    for rec in &events.0 {
      println!("  - {:?}: {:?}", rec.name(), rec.short_doc());
      match &rec.event {
        RuntimeEvent::Balances(events::BalancesEvent::Transfer(
          from_did,
          from,
          to_did,
          to,
          value,
          memo,
        )) => {
          println!(
            "    - balances: transfer({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            from_did, from, to_did, to, value, memo
          );
        }
        RuntimeEvent::Balances(ev) => {
          println!("    - balances: other event: {ev:?}");
        }
        ev => {
          println!("    - other: {ev:?}");
        }
      }
    }
  }
  Ok(())
}

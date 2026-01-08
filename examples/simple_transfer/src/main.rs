use std::env;

use anyhow::Result;

use sp_keyring::Sr25519Keyring;

use polymesh_api::client::PairSigner;
use polymesh_api::polymesh::types::runtime::{events, RuntimeEvent};
use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let mut alice = PairSigner::new(subxt_signer::sr25519::dev::alice());

  let api = Api::new(&url).await?;
  println!("Connection with chain established.");

  let dest = Sr25519Keyring::Bob.to_account_id().into();
  let mut res = api
    .call()
    .balances()
    .transfer_with_memo(dest, 123_012_345, None)?
    .submit_and_watch(&mut alice)
    .await?;

  println!("Transfer submitted, waiting for transaction to be finalized");
  res.wait_finalized().await?;
  println!("POLYX transfer finalized.");

  let events = res.events().await?;
  if let Some(events) = events {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Balances(events::BalancesEvent::TransferWithMemo {
          from,
          to,
          amount,
          memo,
        }) => {
          println!("{} transfered {:?} to {} with memo {:?}", from, amount, to, memo);
        }
        // Ignore other events.
        _ => (),
      }
    }
  }
  Ok(())
}

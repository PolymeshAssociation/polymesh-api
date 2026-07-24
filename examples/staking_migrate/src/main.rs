use std::env;

use futures_util::StreamExt;

use anyhow::Result;

use sp_keyring::Sr25519Keyring;

use polymesh_api::Api;
use polymesh_api_tester::AccountSigner;

const STAKING_LOCK_ID: [u8; 8] = [115, 116, 97, 107, 105, 110, 103, 32];

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  println!("Connecting to {url} ...");

  let signer = if let Some(seed) = env::args().nth(2) {
    AccountSigner::from_string(&seed)?
  } else {
    AccountSigner::new(Sr25519Keyring::Alice.pair())
  };

  let api = Api::new(&url).await?;

  let mut count_ok = 0;
  let mut count_err = 0;
  let mut wait_for_txs = async move |txs: Vec<_>| -> Result<()> {
    println!("Waiting for {} transactions to be finalized...", txs.len());

    for task in txs {
      let result = task.await?;
      match result {
        Ok(_) => {
          count_ok += 1;

          if count_ok % 10 == 0 {
            println!("Transactions finalized: {count_ok} ok, {count_err} error(s)");
          }
        }
        Err(err) => {
          count_err += 1;
          println!("Transaction error: {err:?}");
        }
      }
    }
    println!("Transactions finalized: {count_ok} ok, {count_err} error(s)");

    Ok(())
  };

  // Query locks from the balances pallet.
  let locks = api.paged_query().balances().locks().entries();
  tokio::pin!(locks);
  println!("paged_query.locks.entries:");
  let mut count = 0;
  let mut txs = Vec::new();
  while let Some(lock) = locks.next().await {
    let (id, locks) = lock?;
    let Some(locks) = locks else {
      println!(" -- [{id}] = None");
      continue;
    };

    // Check if the lock is a staking lock.
    if !locks.iter().any(|l| l.id == STAKING_LOCK_ID) {
      println!(" -- [{id}] = {locks:?} (no staking lock so skipping)");
      continue;
    }

    println!(" -- [{id}] = {locks:?}");
    count += 1;

    //if count <= 10 {
    let mut signer = signer.clone();
    let api = api.clone();
    let task = tokio::spawn(async move {
      let mut tx = api
        .call()
        .staking()
        .migrate_currency(id)?
        .submit_and_watch(&mut signer)
        .await?;
      tx.wait_finalized().await?;
      let result = tx.ok().await;
      Ok::<_, anyhow::Error>(result)
    });
    txs.push(task);

    if txs.len() >= 400 {
      wait_for_txs(txs).await?;
      txs = Vec::new();
    }
    //}
  }

  println!("Total locks: {count}");

  wait_for_txs(txs).await?;
  println!("Transactions finalized: {count_ok} ok, {count_err} error(s)");

  Ok(())
}

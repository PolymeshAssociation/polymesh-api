#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use std::env;

use anyhow::Result;

use sp_keyring::Sr25519Keyring;

pub mod polymesh;
use polymesh::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let api = Api::new(&url).await?;

  let dest = Sr25519Keyring::Bob.to_account_id();
  let call = api.call().balances().transfer(dest.into(), 123_012_345);
  println!("balances.transfer = {call:#?}");
  Ok(())
}

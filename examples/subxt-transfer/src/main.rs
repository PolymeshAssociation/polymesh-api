use std::env;
use anyhow::Result;
use subxt::{
    SubstrateConfig,
    OnlineClient,
};
use subxt_signer::sr25519::dev::{self};

#[subxt::subxt(runtime_metadata_path = "src/polymesh_metadata.scale")]
pub mod polymesh {}

use polymesh::balances::events::Transfer;

// SubstrateConfig will suffice for this example.
type PolymeshConfig = SubstrateConfig;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let url = env::args().nth(1).expect("Missing node ws url");

    let api = OnlineClient::<PolymeshConfig>::from_url(&url).await?;
    println!("Connection with chain established.");

    let alice = dev::alice();

    // Transfer POLYX to Bob
    let dest = dev::bob().public_key().into();
    let transfer_tx = polymesh::tx()
        .balances()
        .transfer(dest, 123_012_345);
    let res = api
        .tx()
        .sign_and_submit_then_watch_default(&transfer_tx, &alice)
        .await?;
    println!("Transfer submitted, waiting for transaction to be finalized...");

    let events = res.wait_for_finalized_success()
        .await?;
    println!("POLYX transfer finalized.");

    for evt in events.iter() {
      // try to parse the current event into a Transfer Event
      let parsed_transfer = evt?.as_event::<Transfer>()?;
      
      // check if we have some valid transfer 
      match parsed_transfer {
          Some(Transfer(_from_did, from, _to_did, to, amount, _memo)) => {
            println!("{} transfered {:?} to {}",  from, amount, to);
          }
          // Ignore other events.
          _ => (),
      }
    }

    Ok(())
}

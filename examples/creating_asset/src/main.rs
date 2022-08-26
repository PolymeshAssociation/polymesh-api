use std::env;

use sp_keyring::AccountKeyring;

use polymesh_api::client::{PairSigner, AccountId};
use polymesh_api::Api;

// Types
use polymesh_api::types::polymesh_primitives::asset::AssetType;
use polymesh_api::types::polymesh_primitives::ticker::Ticker;
use polymesh_api::types::pallet_asset::SecurityToken;
use polymesh_api::types::polymesh_primitives::secondary_key::KeyRecord;
use polymesh_api::types::polymesh_primitives::secondary_key::KeyRecord::PrimaryKey;
use polymesh_api::types::polymesh_primitives::secondary_key::KeyRecord::SecondaryKey; 
use polymesh_api::types::polymesh_primitives::secondary_key::KeyRecord::MultiSigSignerKey;
use polymesh_api::types::polymesh_primitives::asset::AssetName;

// Structs
use polymesh_api::client::IdentityId;

use anyhow::{anyhow, Error, Result};

const TOTAL_SUPPLY: u128 = 1_000_000_000u128;
const TICKER_LEN: usize = 12;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let api: Api = Api::new(&url).await?;

  let mut alice = PairSigner::new(AccountKeyring::Alice.pair());
  let record_option = api.query().identity().key_records(alice.account).await?;
  let record: KeyRecord<AccountId> = record_option.unwrap();
 
  let alice_did = key_to_identity_ids(record);
  
  let (ticker, token) = a_token(alice_did);
  let asset_name = as_ref(&ticker);

  let mut res = api
    .call()
    .asset()
    .create_asset(
      AssetName(asset_name.to_vec()),
      ticker,
      token.divisible,
      token.asset_type.clone(),
      vec![],
      None,
      true)?
    .sign_submit_and_watch(&mut alice)
    .await?;
  let events = res.events().await?;
  println!("call1 events = {:#?}", events);

  let result = res.extrinsic_result().await?;
  println!("call result = {:#?}", result);
  
  Ok(())
}

pub fn key_to_identity_ids(record: KeyRecord<AccountId>) -> IdentityId {
  match record {
    PrimaryKey(did) => {
      println!("primary: {:#?}", did); 
      return did;
    },
    SecondaryKey(did, perms) => {
      println!("secondary: {:#?} perms: {:#?}", did, perms);
      return did;
    },
    MultiSigSignerKey(_) => todo!(),
  };
}

fn token(name: &[u8], owner_did: IdentityId) -> (Ticker, SecurityToken) {
  let ticker = try_from(name).unwrap();
  let token = SecurityToken {
      owner_did,
      total_supply: TOTAL_SUPPLY,
      divisible: true,
      asset_type: AssetType::EquityCommon,
  };
  (ticker, token)
}

fn a_token(owner_did: IdentityId) -> (Ticker, SecurityToken) {
  token(b"A", owner_did)
}


fn try_from(s: &[u8]) -> Result<Ticker, Error> {
  let len = s.len();
  if len > TICKER_LEN {
      return Err(anyhow!("ticker too long"));
  }
  let mut inner = [0u8; TICKER_LEN];
  inner[..len].copy_from_slice(s);
  inner.make_ascii_uppercase();
  // Check whether the given ticker contains no lowercase characters and return an error
  // otherwise.
  if &inner[..len] == s {
      Ok(Ticker(inner))
  } else {
      Err(anyhow!("lowercase ticker"))
  }
}

fn as_ref(ticker: &Ticker) -> &[u8] {
  &ticker.0
}


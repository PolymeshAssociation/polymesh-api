//! Tests for the `TransactionPaymentApi`/`TransactionPaymentCallApi` Runtime API queries.
//!
//! These tests require a running Polymesh node, reachable via `POLYMESH_NODE_URL`
//! (defaults to `ws://localhost:9944`).
use std::env;

use anyhow::Result;
use codec::Encode;

use polymesh_api::client::{dev, ChainApi, Signer};
use polymesh_api::Api;

fn node_url() -> String {
  env::var("POLYMESH_NODE_URL").unwrap_or_else(|_| "ws://localhost:9944".into())
}

async fn connect() -> Result<Api> {
  dotenv::dotenv().ok();
  Ok(Api::new(&node_url()).await?)
}

#[tokio::test]
async fn query_transaction_fee_info_for_remark() -> Result<()> {
  let api = connect().await?;
  let client = api.client();

  // `query_info`/`query_fee_details` only compute a `partial_fee` for signed extrinsics.
  let mut alice = dev::alice();
  let call = api.call().system().remark(b"polymesh-api test".to_vec())?;
  let xt = call
    .prepare(alice.account(), None)
    .await?
    .sign(&mut alice)
    .await?;
  let encoded_xt = xt.encode();

  let info = client
    .query_transaction_fee_info::<u128>(&encoded_xt, None)
    .await?;
  assert!(info.weight.ref_time() > 0);
  assert!(info.partial_fee > 0);

  let details = client
    .query_transaction_fee_details::<u128>(&encoded_xt, None)
    .await?;
  assert!(details.inclusion_fee.is_some());

  Ok(())
}

#[tokio::test]
async fn query_call_fee_info_for_remark() -> Result<()> {
  let api = connect().await?;
  let client = api.client();

  let call = api.call().system().remark(b"polymesh-api test".to_vec())?;

  let info = client
    .query_call_fee_info::<_, u128>(call.runtime_call(), None)
    .await?;
  assert!(info.weight.ref_time() > 0);
  assert!(info.partial_fee > 0);

  let details = client
    .query_call_fee_details::<_, u128>(call.runtime_call(), None)
    .await?;
  assert!(details.inclusion_fee.is_some());

  Ok(())
}

use anyhow::Result;

use polymesh_api_client_extras::*;
use polymesh_api_tester::*;

#[tokio::test]
async fn create_secondary_keys() -> Result<()> {
  let mut tester = PolymeshTester::new().await?;
  let users = tester
    .users_with_secondary_keys(&[("UserHas2SecondaryKeys", 2), ("UserHas10SecondaryKeys", 10)])
    .await?;

  let mut results = Vec::new();
  for mut user in users {
    let pk = user.account();
    for sk in &mut user.secondary_keys {
      // Send some POLYX from the primary key to the secondary key.
      let res = tester
        .api
        .call()
        .balances()
        .transfer_with_memo(sk.account().into(), ONE_POLYX, None)?
        .submit_and_watch(&mut user.primary_key)
        .await?;
      results.push(res);
      // Send some POLYX back to the primary key from the secondary key.
      let res = tester
        .api
        .call()
        .balances()
        .transfer_with_memo(pk.into(), ONE_POLYX, None)?
        .submit_and_watch(sk)
        .await?;
      results.push(res);
    }
  }
  // Wait for all results.
  for mut res in results {
    println!("transfer res = {:#?}", res.ok().await);
  }
  Ok(())
}

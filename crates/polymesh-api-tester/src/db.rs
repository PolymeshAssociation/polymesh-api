use sqlx::sqlite::*;

use polymesh_api::client::AccountId;
use polymesh_api::{Api, ChainApi};

use crate::error::Result;

#[derive(Clone)]
pub struct Db {
  api: Api,
  pool: SqlitePool,
}

impl Db {
  pub async fn open(api: Api, file: &str) -> Result<Self> {
    let pool = SqlitePool::connect(file).await?;
    Ok(Self { api, pool })
  }

  pub async fn get_next_nonce(&self, account: AccountId) -> Result<u32> {
    // Get the nonce from the chain.  (to check if the db nonce is old).
    let nonce = self.api.get_nonce(account).await?;

    let id = account.to_string();
    // Save the nonce to the database.
    let rec = sqlx::query!(
      r#"
      INSERT INTO accounts(account, nonce) VALUES(?, ?)
        ON CONFLICT(account) DO UPDATE SET nonce=MAX(nonce+1, excluded.nonce)
      RETURNING nonce
      "#,
      id,
      nonce
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(rec.nonce as u32)
  }
}

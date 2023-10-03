use sqlx::SqlitePool;

use polymesh_api::client::AccountId;

use crate::error::Result;

#[derive(Clone)]
pub struct Db {
  pool: SqlitePool,
}

impl Db {
  pub async fn open(file: &str) -> Result<Self> {
    let pool = SqlitePool::connect(file).await?;
    Ok(Self { pool })
  }

  pub async fn get_nonce(&self, account: AccountId) -> Result<u32> {
    let id = account.to_string();
    let row = sqlx::query!(
      r#"
      INSERT INTO accounts(account) VALUES(?)
        ON CONFLICT(account) DO UPDATE SET nonce=nonce+1
      RETURNING nonce
    "#,
      id
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(row.nonce as u32)
  }
}

use core::ops::{Deref, DerefMut};

use polymesh_api::{
  client::{dev, AccountId, DefaultSigner, IdentityId, Result, Signer},
  polymesh::types::{
    frame_system::AccountInfo,
    polymesh_primitives::secondary_key::KeyRecord,
  },
  Api,
};
#[cfg(not(feature = "polymesh_v8"))]
use polymesh_api::polymesh::types::pallet_balances::AccountData;
#[cfg(feature = "polymesh_v8")]
use polymesh_api::polymesh::types::pallet_balances::types::AccountData;

use crate::*;

#[derive(Clone)]
pub struct PolymeshHelper {
  api: Api,
  pub init_polyx: u128,
  pub cdd: DefaultSigner,
}

impl PolymeshHelper {
  pub async fn new(url: &str) -> Result<Self> {
    let api = Api::new(url).await?;
    Ok(Self {
      api,
      init_polyx: 100_000 * ONE_POLYX,
      cdd: dev::alice(),
    })
  }

  async fn batch_load_did_balance(&self, users: &mut [PolymeshUser]) -> Result<Vec<u128>> {
    let mut balances = Vec::with_capacity(users.len());
    for users in users.chunks_mut(200) {
      let mut tasks = Vec::with_capacity(200);
      for user in users.iter() {
        let account = user.account();
        let need_did = user.did.is_none();
        let helper = self.clone();
        let task = tokio::spawn(async move { helper.get_did_and_balance(account, need_did).await });
        tasks.push(task);
      }
      for (idx, task) in tasks.into_iter().enumerate() {
        let (did, balance) = task.await.unwrap()?;
        if did.is_some() {
          users[idx].did = did;
        }
        balances.push(balance);
      }
    }
    Ok(balances)
  }

  async fn batch_load_dids(&self, users: &mut [PolymeshUser]) -> Result<()> {
    for users in users.chunks_mut(200) {
      let mut tasks = Vec::with_capacity(200);
      for (idx, user) in users.iter().enumerate() {
        let account = user.account();
        let helper = self.clone();
        if user.did.is_none() {
          let task = tokio::spawn(async move { helper.get_did(account).await });
          tasks.push((idx, task));
        }
      }
      for (idx, task) in tasks {
        let did = task.await.unwrap()?;
        if did.is_some() {
          users[idx].did = did;
        }
      }
    }
    Ok(())
  }

  /// Generate new users from names.
  ///
  /// The users will be onboarded with the CDD provider, if they don't have an identity.
  /// Also they will be funded with `init_polyx` POLYX.
  pub async fn generate_named_users(&mut self, names: &[&str]) -> Result<Vec<PolymeshUser>> {
    let mut users = Vec::with_capacity(names.len());
    for name in names {
      // Get or create user.
      users.push(PolymeshUser::new(name)?);
    }
    self.onboard_users(&mut users).await?;
    Ok(users)
  }

  /// Generate new users from prefix.
  ///
  /// The users will be onboarded with the CDD provider, if they don't have an identity.
  /// Also they will be funded with `init_polyx` POLYX.
  pub async fn generate_prefix_users(
    &mut self,
    prefix: &str,
    count: usize,
  ) -> Result<Vec<PolymeshUser>> {
    let mut users = Vec::with_capacity(count);
    for idx in 0..count {
      // Get or create user.
      users.push(PolymeshUser::new(&format!("{prefix}_{idx}"))?);
    }
    self.onboard_users(&mut users).await?;
    Ok(users)
  }

  async fn onboard_users(&mut self, users: &mut [PolymeshUser]) -> Result<()> {
    let mut batches = Vec::new();
    for users in users.chunks_mut(200) {
      let balances = self.batch_load_did_balance(users).await?;
      // Calls for registering users and funding them.
      let mut did_calls = Vec::new();
      let mut fund_calls = Vec::new();
      for (idx, user) in users.iter_mut().enumerate() {
        let account = user.account();
        // If the user doesn't have an identity, then register them.
        if user.did.is_none() {
          // User needs an identity.
          did_calls.push(
            self
              .api
              .call()
              .identity()
              .cdd_register_did_with_cdd(account, vec![], None)?
              .into(),
          );
        }
        // Add calls to fund the users.
        let balance = balances[idx];
        if balance < self.init_polyx {
          // Transfer some funds to the user.
          #[cfg(not(feature = "polymesh_v8"))]
          fund_calls.push(
            self
              .api
              .call()
              .balances()
              .set_balance(account.into(), self.init_polyx, 0)?
              .into(),
          );
          #[cfg(feature = "polymesh_v8")]
          fund_calls.push(
            self
              .api
              .call()
              .balances()
              .force_set_balance(account.into(), self.init_polyx)?
              .into(),
          );
        }
      }
      if did_calls.len() > 0 {
        let res = self
          .api
          .call()
          .utility()
          .batch(did_calls)?
          .submit_and_watch(&mut self.cdd)
          .await?;
        batches.push(res);
      }
      if fund_calls.len() > 0 {
        let res = self
          .api
          .call()
          .sudo()
          .sudo(self.api.call().utility().batch(fund_calls)?.into())?
          .submit_and_watch(&mut self.cdd)
          .await?;
        batches.push(res);
      }
    }
    // Check if we have any batches to process.
    if batches.len() == 0 {
      return Ok(());
    }
    // Wait for all batches to be finalized.
    for mut res in batches {
      res.wait_finalized().await?;
    }
    // Get new identities.
    self.batch_load_dids(users).await?;
    Ok(())
  }

  pub async fn key_records(&self, account: AccountId) -> Result<Option<KeyRecord<AccountId>>> {
    Ok(self.api.query().identity().key_records(account).await?)
  }

  pub async fn get_did(&self, account: AccountId) -> Result<Option<IdentityId>> {
    let did = match self.key_records(account).await? {
      Some(KeyRecord::PrimaryKey(did)) => Some(did),
      Some(KeyRecord::SecondaryKey(did)) => Some(did),
      _ => None,
    };
    Ok(did)
  }

  #[cfg(not(feature = "polymesh_v8"))]
  pub async fn get_account_info(
    &self,
    account: AccountId,
  ) -> Result<AccountInfo<u32, AccountData>> {
    Ok(self.api.query().system().account(account).await?)
  }

  #[cfg(feature = "polymesh_v8")]
  pub async fn get_account_info(
    &self,
    account: AccountId,
  ) -> Result<AccountInfo<u32, AccountData<u128>>> {
    Ok(self.api.query().system().account(account).await?)
  }

  pub async fn get_account_balance(&self, account: AccountId) -> Result<u128> {
    self
      .get_account_info(account)
      .await
      .map(|info| info.data.free)
  }

  async fn get_did_and_balance(
    &self,
    account: AccountId,
    need_did: bool,
  ) -> Result<(Option<IdentityId>, u128)> {
    let did: Option<IdentityId> = if need_did {
      self.get_did(account).await?
    } else {
      None
    };
    let balance = self.get_account_balance(account).await?;
    Ok((did, balance))
  }

  pub async fn register_and_fund(&mut self, account: AccountId) -> Result<IdentityId> {
    let did = match self.get_did(account).await? {
      Some(did) => did,
      None => {
        // `account` is not linked to an identity.
        // Create a new identity with `account` as the primary key.
        let mut res = self
          .api
          .call()
          .utility()
          .batch(vec![
            self
              .api
              .call()
              .identity()
              .cdd_register_did_with_cdd(account, vec![], None)?
              .into(),
            self
              .api
              .call()
              .balances()
              .transfer_with_memo(account.into(), self.init_polyx, None)?
              .into(),
          ])?
          .execute(&mut self.cdd)
          .await?;
        get_identity_id(&mut res).await?.unwrap()
      }
    };
    Ok(did)
  }
}

#[derive(Clone)]
pub struct PolymeshUser {
  pub name: String,
  signer: DefaultSigner,
  pub did: Option<IdentityId>,
}

impl Deref for PolymeshUser {
  type Target = DefaultSigner;

  fn deref(&self) -> &Self::Target {
    &self.signer
  }
}

impl DerefMut for PolymeshUser {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.signer
  }
}

impl PolymeshUser {
  pub fn new(name: &str) -> Result<Self> {
    Ok(Self {
      name: name.to_string(),
      signer: DefaultSigner::from_string(&format!("//{name}"), None)?,
      did: None,
    })
  }

  pub fn from_signer(name: &str, signer: DefaultSigner) -> Self {
    Self {
      name: name.to_string(),
      signer,
      did: None,
    }
  }
}

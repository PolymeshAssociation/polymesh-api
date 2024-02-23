use core::ops::{Deref, DerefMut};

use polymesh_api::{
  client::{AccountId, IdentityId, DefaultSigner, Signer, Result, dev},
  polymesh::types::polymesh_primitives::secondary_key::KeyRecord,
  Api,
};

use crate::*;

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

  async fn load_dids(&self, users: &mut [PolymeshUser]) -> Result<()> {
    for user in users {
      // Skip users that have an identity.
      if user.did.is_some() {
        continue;
      }
      // Try getting the user's identity from the chain.
      user.did = self.get_did(user.account()).await?;
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
  pub async fn generate_prefix_users(&mut self, prefix: &str, count: usize) -> Result<Vec<PolymeshUser>> {
    let mut users = Vec::with_capacity(count);
    for idx in 0..count {
      // Get or create user.
      users.push(PolymeshUser::new(&format!("{prefix}_{idx}"))?);
    }
    self.onboard_users(&mut users).await?;
    Ok(users)
  }

  async fn onboard_users(&mut self, users: &mut [PolymeshUser]) -> Result<()> {
    self.load_dids(users).await?;
    // Calls for registering users and funding them.
    let mut calls = Vec::new();
    // Add calls to register users missing identities.
    let mut need_dids = Vec::new();
    for (idx, user) in users.iter().enumerate() {
      if user.did.is_some() {
        continue;
      }
      need_dids.push(idx);
      // User needs an identity.
      calls.push(
        self
          .api
          .call()
          .identity()
          .cdd_register_did_with_cdd(user.account(), vec![], None)?
          .into(),
      );
    }
    // Add calls to fund the users.
    for user in users.iter() {
      // Transfer some funds to the user.
      calls.push(
        self
          .api
          .call()
          .balances()
          .transfer(user.account().into(), self.init_polyx)?
          .into(),
      );
    }
    // Execute batch.
    let mut res = self
      .api
      .call()
      .utility()
      .batch(calls)?
      .execute(&mut self.cdd)
      .await?;
    // Get new identities from batch events.
    let ids = get_created_ids(&mut res).await?;
    for idx in need_dids {
      match &ids[idx] {
        CreatedIds::IdentityCreated(did) => {
          users[idx].did = Some(*did);
        }
        _ => (),
      }
    }
    Ok(())
  }

  pub async fn key_records(&self, account: AccountId) -> Result<Option<KeyRecord<AccountId>>> {
    Ok(self.api.query().identity().key_records(account).await?)
  }

  pub async fn get_did(&self, account: AccountId) -> Result<Option<IdentityId>> {
    let did = match self.key_records(account).await? {
      Some(KeyRecord::PrimaryKey(did)) => Some(did),
      Some(KeyRecord::SecondaryKey(did, _)) => Some(did),
      _ => None,
    };
    Ok(did)
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
              .transfer(account.into(), self.init_polyx)?
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
      did: None
    })
  }

  pub fn from_signer(name: &str, signer: DefaultSigner) -> Self {
    Self {
      name: name.to_string(),
      signer,
      did: None
    }
  }
}

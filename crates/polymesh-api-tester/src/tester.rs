use std::collections::HashMap;

use polymesh_api::{
  client::{AccountId, IdentityId},
  polymesh::types::polymesh_primitives::{
    secondary_key::{KeyRecord, Permissions, SecondaryKey},
    subset::SubsetRestriction,
    ticker::Ticker,
  },
  Api,
};
use polymesh_api_client_extras::*;

use crate::*;

async fn get_sudo_signer(api: &Api, signer: &AccountSigner) -> Option<AccountSigner> {
  api.query().sudo().key().await.ok()?.and_then(|key| {
    if key == signer.account() {
      Some(signer.clone())
    } else {
      None
    }
  })
}

pub struct PolymeshTester {
  pub api: Api,
  seed: String,
  init_polyx: u128,
  db: Db,
  cdd: AccountSigner,
  sudo: Option<AccountSigner>,
  users: HashMap<String, User>,
}

impl PolymeshTester {
  pub async fn new() -> Result<Self> {
    let api = client_api().await?;
    // Generate a seed based on current timestamp.
    // We use a 'seed' to allow running tests in parallel.
    let ts = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("now later then epoch")
      .as_nanos();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "accounts.db".into());
    let db = Db::open(api.clone(), &url).await?;
    let cdd = AccountSigner::alice(Some(db.clone()));
    let sudo = get_sudo_signer(&api, &cdd).await;
    Ok(Self {
      api,
      init_polyx: 10_000 * ONE_POLYX,
      seed: format!("{}", ts),
      cdd,
      sudo,
      users: Default::default(),
      db,
    })
  }

  /// Set how much POLYX to fund each user.
  pub fn set_init_polyx(&mut self, val: u128) {
    self.init_polyx = val * ONE_POLYX;
  }

  pub fn has_sudo(&self) -> bool {
    self.sudo.is_some()
  }

  fn set_user_did(&mut self, name: &str, did: IdentityId) {
    if let Some(user) = self.users.get_mut(name) {
      user.did = Some(did);
    }
  }

  fn update_user(&mut self, name: &str, user: &User) {
    self.users.insert(name.to_string(), user.clone());
  }

  fn new_signer_idx(&self, name: &str, idx: usize) -> Result<AccountSigner> {
    AccountSigner::from_string(
      Some(self.db.clone()),
      &format!("//{}_{}_{}", self.seed, name, idx),
    )
  }

  fn get_user(&mut self, name: &str) -> Result<User> {
    use std::collections::hash_map::Entry;
    match self.users.entry(name.to_string()) {
      Entry::Occupied(entry) => Ok(entry.get().clone()),
      Entry::Vacant(entry) => {
        let signer = AccountSigner::from_string(
          Some(self.db.clone()),
          &format!("//{}_{}", self.seed, entry.key()),
        )?;
        let user = User::new(&self.api, signer);
        Ok(entry.insert(user).clone())
      }
    }
  }

  /// Get the user if they exist, or create a new one.  Make sure the user
  /// has an identity.
  pub async fn user(&mut self, name: &str) -> Result<User> {
    let mut user = self.get_user(name)?;
    if user.did.is_none() {
      let did = self.register_and_fund(user.account()).await?;
      user.did = Some(did);
      self.set_user_did(name, did);
    }
    Ok(user)
  }

  async fn load_dids(&mut self, users: &mut [User]) -> Result<()> {
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

  /// Get the users if they exist, or create them.  Make sure the users
  /// have identities.
  pub async fn users(&mut self, names: &[&str]) -> Result<Vec<User>> {
    let names = names.iter().map(|name| (*name, 0)).collect::<Vec<_>>();
    self.users_with_secondary_keys(names.as_slice()).await
  }

  /// Get the users if they exist, or create them.  Make sure the users
  /// have identities.
  pub async fn users_with_secondary_keys(&mut self, names: &[(&str, usize)]) -> Result<Vec<User>> {
    let mut users = Vec::new();
    let mut accounts = Vec::new();
    for (name, sk) in names {
      // Get or create user.
      let mut user = self.get_user(name)?;
      if user.secondary_keys.len() < *sk {
        for idx in 0..*sk {
          let sk = self.new_signer_idx(name, idx)?;
          accounts.push(sk.account());
          user.secondary_keys.push(sk);
        }
        self.update_user(name, &user);
      }
      accounts.push(user.account());
      users.push(user);
    }
    self.load_dids(users.as_mut_slice()).await?;
    // Calls for registering users and funding them.
    let mut calls = Vec::new();
    let has_sudo = self.has_sudo();
    // Add calls to register users missing identities.
    let mut need_dids = Vec::new();
    let mut has_secondary_keys = false;
    for (idx, user) in users.iter().enumerate() {
      if user.did.is_some() {
        continue;
      }
      need_dids.push(idx);
      if user.secondary_keys.len() > 0 {
        has_secondary_keys = true;
      }
      let secondary_keys = user
        .secondary_keys
        .iter()
        .map(|key| SecondaryKey {
          key: key.account(),
          permissions: Permissions {
            asset: SubsetRestriction::Whole,
            extrinsic: SubsetRestriction::Whole,
            portfolio: SubsetRestriction::Whole,
          },
        })
        .collect();
      // User needs an identity.
      calls.push(
        self
          .api
          .call()
          .identity()
          .cdd_register_did_with_cdd(user.account(), secondary_keys, None)?
          .into(),
      );
    }
    // Add calls to fund the users.
    let mut sudos = Vec::new();
    for account in accounts {
      if has_sudo {
        // We have sudo, just set their balance.
        sudos.push(
          self
            .api
            .call()
            .balances()
            .set_balance(account.into(), self.init_polyx, 0)?
            .into(),
        );
      } else {
        // Transfer some funds to the user.
        calls.push(
          self
            .api
            .call()
            .balances()
            .transfer(account.into(), self.init_polyx)?
            .into(),
        );
      }
    }
    let signer = self.sudo.as_mut().unwrap_or_else(|| &mut self.cdd);
    // Execute batch.
    let mut res = self
      .api
      .call()
      .utility()
      .batch(calls)?
      .submit_and_watch(signer)
      .await?;
    let sudo_res = if sudos.len() > 0 {
      Some(
        self
          .api
          .call()
          .sudo()
          .sudo(self.api.call().utility().batch(sudos)?.into())?
          .submit_and_watch(signer)
          .await?,
      )
    } else {
      None
    };
    let mut auths = HashMap::new();
    if has_secondary_keys {
      if let Some(events) = res.events().await? {
        for rec in &events.0 {
          match &rec.event {
            RuntimeEvent::Identity(IdentityEvent::AuthorizationAdded(
              _,
              _,
              Some(sk),
              auth,
              _,
              _,
            )) => {
              auths.insert(*sk, *auth);
            }
            _ => (),
          }
        }
      }
    }
    // Get new identities from batch events.
    let ids = get_created_ids(&mut res).await?;
    for idx in need_dids {
      let (name, _) = names[idx];
      match &ids[idx] {
        CreatedIds::IdentityCreated(did) => {
          let user = &mut users[idx];
          user.did = Some(*did);
          for sk in &mut user.secondary_keys {
            if let Some(auth) = auths.remove(&sk.account()) {
              self
                .api
                .call()
                .identity()
                .join_identity_as_key(auth)?
                .submit_and_watch(sk)
                .await?;
            }
          }
          self.set_user_did(name, *did);
        }
        id => {
          log::warn!("Unexpected id: {id:?}");
        }
      }
    }
    // Wait for both batches to finalize.
    if let Some(mut res) = sudo_res {
      res.wait_finalized().await?;
    }
    res.wait_finalized().await?;
    Ok(users)
  }

  pub async fn key_records(&self, account: AccountId) -> Result<Option<KeyRecord<AccountId>>> {
    Ok(self.api.query().identity().key_records(account).await?)
  }

  pub async fn get_did(&self, account: AccountId) -> Result<Option<IdentityId>> {
    let did = match self.key_records(account).await? {
      Some(KeyRecord::PrimaryKey(did)) => Some(did),
      #[cfg(feature = "v6")]
      Some(KeyRecord::SecondaryKey(did, _)) => Some(did),
      #[cfg(feature = "v7")]
      Some(KeyRecord::SecondaryKey(did)) => Some(did),
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

  pub fn gen_ticker(&self) -> Ticker {
    use rand::{thread_rng, Rng};
    let mut data = [0u8; 6];
    thread_rng().fill(&mut data[..]);
    let name = hex::encode_upper(data);
    Ticker(name.as_bytes().try_into().unwrap())
  }
}

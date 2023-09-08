#[cfg(feature = "std")]
use sp_core::Pair;

use sp_runtime::MultiSignature;
use sp_std::prelude::*;

use async_trait::async_trait;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

use crate::*;

#[cfg(feature = "std")]
pub type DefaultSigner = PairSigner<sp_core::sr25519::Pair>;
#[cfg(not(feature = "std"))]
pub type DefaultSigner = PairSigner<subxt_signer::sr25519::Keypair>;

#[async_trait]
pub trait Signer {
  fn account(&self) -> AccountId;

  /// Optional - The signer can manage their `nonce` for improve transaction performance.
  /// The default implmentation will query the next `nonce` from chain storage.
  fn nonce(&self) -> Option<u32> {
    None
  }

  /// Optional - The signer can manage their `nonce` for improve transaction performance.
  /// If the transaction is accepted by the RPC node, then the `nonce` we be increased, to
  /// allow the next transaction to be signed & submitted without waiting for the next block.
  fn set_nonce(&mut self, _nonce: u32) {}

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature>;
}

pub trait KeypairSigner: Send + Sync + Sized {
  fn account(&self) -> AccountId;
  fn sign(&self, message: &[u8]) -> MultiSignature;
  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self>;
}

#[cfg(feature = "std")]
impl KeypairSigner for sp_core::ed25519::Pair {
  fn account(&self) -> AccountId {
    self.public().into()
  }

  fn sign(&self, message: &[u8]) -> MultiSignature {
    <sp_core::ed25519::Pair as sp_core::Pair>::sign(self, message).into()
  }

  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    Ok(<sp_core::ed25519::Pair as sp_core::Pair>::from_string(
      s,
      password_override,
    )?)
  }
}

#[cfg(feature = "std")]
impl KeypairSigner for sp_core::sr25519::Pair {
  fn account(&self) -> AccountId {
    self.public().into()
  }

  fn sign(&self, message: &[u8]) -> MultiSignature {
    <sp_core::sr25519::Pair as sp_core::Pair>::sign(self, message).into()
  }

  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    Ok(<sp_core::sr25519::Pair as sp_core::Pair>::from_string(
      s,
      password_override,
    )?)
  }
}

impl KeypairSigner for subxt_signer::sr25519::Keypair {
  fn account(&self) -> AccountId {
    AccountId(self.public_key().0)
  }

  fn sign(&self, message: &[u8]) -> MultiSignature {
    let sig = subxt_signer::sr25519::Keypair::sign(self, message).0;
    MultiSignature::Sr25519(sp_core::sr25519::Signature(sig))
  }

  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    use alloc::str::FromStr;
    let mut uri = subxt_signer::SecretUri::from_str(s)?;
    if let Some(password_override) = password_override {
      uri.password = Some(password_override.to_string().into());
    }
    Ok(subxt_signer::sr25519::Keypair::from_uri(&uri)?)
  }
}

impl KeypairSigner for subxt_signer::ecdsa::Keypair {
  fn account(&self) -> AccountId {
    let pub_key = self.public_key();
    let hash = sp_core::hashing::blake2_256(&pub_key.0[..]);
    AccountId(hash)
  }

  fn sign(&self, message: &[u8]) -> MultiSignature {
    let sig = subxt_signer::ecdsa::Keypair::sign(self, message).0;
    MultiSignature::Ecdsa(sp_core::ecdsa::Signature(sig))
  }

  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    use alloc::str::FromStr;
    let mut uri = subxt_signer::SecretUri::from_str(s)?;
    if let Some(password_override) = password_override {
      uri.password = Some(password_override.to_string().into());
    }
    Ok(subxt_signer::ecdsa::Keypair::from_uri(&uri)?)
  }
}

pub struct PairSigner<P: KeypairSigner> {
  pub pair: P,
  pub nonce: u32,
  pub account: AccountId,
}

impl<P> PairSigner<P>
where
  P: KeypairSigner,
{
  pub fn new(pair: P) -> Self {
    let account = pair.account();
    Self {
      pair,
      nonce: 0,
      account,
    }
  }

  /// Generate signing key pair from string `s`.
  pub fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    Ok(Self::new(P::from_string(s, password_override)?))
  }
}

#[async_trait]
impl<P: KeypairSigner> Signer for PairSigner<P> {
  fn account(&self) -> AccountId {
    self.account.clone()
  }

  fn nonce(&self) -> Option<u32> {
    if self.nonce > 0 {
      Some(self.nonce)
    } else {
      None
    }
  }

  fn set_nonce(&mut self, nonce: u32) {
    self.nonce = nonce;
  }

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature> {
    Ok(self.pair.sign(msg))
  }
}

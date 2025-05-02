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

pub mod dev {
  use super::DefaultSigner;

  pub fn alice() -> DefaultSigner {
    DefaultSigner::from_string("//Alice", None).expect("Const seed")
  }

  pub fn bob() -> DefaultSigner {
    DefaultSigner::from_string("//Bob", None).expect("Const seed")
  }

  pub fn charlie() -> DefaultSigner {
    DefaultSigner::from_string("//Charlie", None).expect("Const seed")
  }

  pub fn dave() -> DefaultSigner {
    DefaultSigner::from_string("//Dave", None).expect("Const seed")
  }

  pub fn eve() -> DefaultSigner {
    DefaultSigner::from_string("//Eve", None).expect("Const seed")
  }

  pub fn ferdie() -> DefaultSigner {
    DefaultSigner::from_string("//Ferdie", None).expect("Const seed")
  }

  pub fn one() -> DefaultSigner {
    DefaultSigner::from_string("//One", None).expect("Const seed")
  }

  pub fn two() -> DefaultSigner {
    DefaultSigner::from_string("//Two", None).expect("Const seed")
  }

  pub fn alice_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Alice//stash", None).expect("Const seed")
  }

  pub fn bob_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Bob//stash", None).expect("Const seed")
  }

  pub fn charlie_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Charlie//stash", None).expect("Const seed")
  }

  pub fn dave_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Dave//stash", None).expect("Const seed")
  }

  pub fn eve_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Eve//stash", None).expect("Const seed")
  }

  pub fn ferdie_stash() -> DefaultSigner {
    DefaultSigner::from_string("//Ferdie//stash", None).expect("Const seed")
  }
}

#[async_trait]
pub trait Signer: Send + Sync {
  fn account(&self) -> AccountId;

  /// Optional - The signer can manage their `nonce` for improve transaction performance.
  /// The default implmentation will query the next `nonce` from chain storage.
  async fn nonce(&self) -> Option<u32> {
    None
  }

  /// Optional - The signer can manage their `nonce` for improve transaction performance.
  /// If the transaction is accepted by the RPC node, then the `nonce` we be increased, to
  /// allow the next transaction to be signed & submitted without waiting for the next block.
  async fn set_nonce(&mut self, _nonce: u32) {}

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature>;

  /// Optional support for locking the signer.
  async fn lock(&self) -> Option<Box<dyn Signer>> {
    None
  }
}

pub trait KeypairSigner: Send + Sync + Sized + Clone {
  fn account(&self) -> AccountId;
  fn sign(&self, message: &[u8]) -> MultiSignature;
  fn from_string(s: &str, password_override: Option<&str>) -> Result<Self>;

  fn verify<M: AsRef<[u8]>>(&self, _sig: &MultiSignature, _message: M) -> Result<bool> {
    unimplemented!()
  }
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

  fn verify<M: AsRef<[u8]>>(&self, sig: &MultiSignature, message: M) -> Result<bool> {
    let sig = match sig {
      MultiSignature::Ed25519(sig) => sig,
      _ => {
        return Err(Error::CoreCryptoError(format!(
          "Invalid signature type: {sig:?}"
        )))
      }
    };
    Ok(<sp_core::ed25519::Pair as sp_core::Pair>::verify(
      sig,
      message.as_ref(),
      &self.public(),
    ))
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

  fn verify<M: AsRef<[u8]>>(&self, sig: &MultiSignature, message: M) -> Result<bool> {
    let sig = match sig {
      MultiSignature::Sr25519(sig) => sig,
      _ => {
        return Err(Error::CoreCryptoError(format!(
          "Invalid signature type: {sig:?}"
        )))
      }
    };
    Ok(<sp_core::sr25519::Pair as sp_core::Pair>::verify(
      sig,
      message.as_ref(),
      &self.public(),
    ))
  }
}

impl KeypairSigner for subxt_signer::sr25519::Keypair {
  fn account(&self) -> AccountId {
    AccountId(self.public_key().0)
  }

  fn sign(&self, message: &[u8]) -> MultiSignature {
    let sig = subxt_signer::sr25519::Keypair::sign(self, message).0;
    MultiSignature::Sr25519(sig.into())
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
    MultiSignature::Ecdsa(sig.into())
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

#[derive(Clone)]
pub struct PairSigner<P: KeypairSigner + Clone> {
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
impl Signer for Box<dyn Signer> {
  fn account(&self) -> AccountId {
    self.as_ref().account()
  }

  async fn nonce(&self) -> Option<u32> {
    self.as_ref().nonce().await
  }

  async fn set_nonce(&mut self, nonce: u32) {
    self.as_mut().set_nonce(nonce).await
  }

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature> {
    self.as_ref().sign(msg).await
  }

  async fn lock(&self) -> Option<Box<dyn Signer>> {
    self.as_ref().lock().await
  }
}

#[async_trait]
impl<P: KeypairSigner> Signer for PairSigner<P> {
  fn account(&self) -> AccountId {
    self.account.clone()
  }

  async fn nonce(&self) -> Option<u32> {
    if self.nonce > 0 {
      Some(self.nonce)
    } else {
      None
    }
  }

  async fn set_nonce(&mut self, nonce: u32) {
    self.nonce = nonce;
  }

  async fn sign(&self, msg: &[u8]) -> Result<MultiSignature> {
    Ok(self.pair.sign(msg))
  }
}

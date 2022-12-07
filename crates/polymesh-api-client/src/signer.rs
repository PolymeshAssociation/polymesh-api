use sp_core::{sr25519, Pair};
use sp_runtime::MultiSignature;

use async_trait::async_trait;

use crate::*;

pub type DefaultSigner = PairSigner<sr25519::Pair>;

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

pub struct PairSigner<P: Pair> {
  pub pair: P,
  pub nonce: u32,
  pub account: AccountId,
}

impl<P> PairSigner<P>
where
  P: Pair,
  MultiSignature: From<<P as Pair>::Signature>,
  AccountId: From<<P as Pair>::Public>,
{
  pub fn new(pair: P) -> Self {
    let account = pair.public().into();
    Self {
      pair,
      nonce: 0,
      account,
    }
  }

  /// Generate signing key pair from string `s`.
  ///
  /// See [`from_string_with_seed`](Pair::from_string_with_seed) for more extensive documentation.
  pub fn from_string(s: &str, password_override: Option<&str>) -> Result<Self> {
    Ok(Self::new(P::from_string(s, password_override)?))
  }
}

#[async_trait]
impl<P: Pair> Signer for PairSigner<P>
where
  MultiSignature: From<<P as Pair>::Signature>,
{
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
    Ok(self.pair.sign(msg).into())
  }
}

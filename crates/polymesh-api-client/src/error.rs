#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(not(feature = "std"))]
use alloc::{fmt, format, string::String};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum Error {
  #[cfg_attr(feature = "std", error("Std io error: {0}"))]
  #[cfg(feature = "std")]
  StdIo(std::io::Error),

  #[cfg_attr(feature = "std", error("Json error: {0}"))]
  Json(serde_json::Error),

  #[cfg_attr(feature = "std", error("hex error: {0}"))]
  Hex(hex::FromHexError),

  #[cfg_attr(feature = "std", error("http error: {0}"))]
  Http(http::Error),

  #[cfg_attr(feature = "std", error("http uri error: {0}"))]
  HttpUri(http::uri::InvalidUri),

  #[cfg_attr(feature = "std", error("parity-scale-codec error: {0}"))]
  ParityScaleCodec(codec::Error),

  #[cfg_attr(feature = "std", error("sp-core crypto secret error: {0}"))]
  SecretStringError(String),

  #[cfg_attr(feature = "std", error("sp-core crypto error: {0}"))]
  CoreCryptoError(String),

  #[cfg_attr(
    feature = "std",
    error("Call API incompatible with connected chain: {0}")
  )]
  IncompatibleCall(String),

  #[cfg_attr(feature = "std", error("Schema failed to parse: {0}"))]
  SchemaParseFailed(String),

  #[cfg_attr(feature = "std", error("Metadata failed to parse: {0}"))]
  MetadataParseFailed(String),

  #[cfg_attr(feature = "std", error("ExtrinsicError: {0}"))]
  ExtrinsicError(String),

  #[cfg_attr(feature = "std", error("RpcClient: {0}"))]
  RpcClient(String),

  #[cfg_attr(feature = "std", error("Decode type failed: {0}"))]
  DecodeTypeFailed(String),

  #[cfg_attr(feature = "std", error("Encode type failed: {0}"))]
  EncodeTypeFailed(String),

  #[cfg_attr(feature = "std", error("Signing transaction failed: {0}"))]
  SigningTransactionFailed(String),

  #[cfg_attr(feature = "std", error("Jsonrpsee error: {0}"))]
  Jsonrpsee(jsonrpsee::core::Error),

  #[cfg_attr(
    feature = "std",
    error("The signer's account {0} doesn't match the transaction's account: {1}")
  )]
  WrongSignerAccount(String, String),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Self::StdIo(e)
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Self::Json(e)
  }
}

impl From<hex::FromHexError> for Error {
  fn from(e: hex::FromHexError) -> Self {
    Self::Hex(e)
  }
}

impl From<http::Error> for Error {
  fn from(e: http::Error) -> Self {
    Self::Http(e)
  }
}

impl From<http::uri::InvalidUri> for Error {
  fn from(e: http::uri::InvalidUri) -> Self {
    Self::HttpUri(e)
  }
}

impl From<codec::Error> for Error {
  fn from(e: codec::Error) -> Self {
    Self::ParityScaleCodec(e)
  }
}

impl From<jsonrpsee::core::Error> for Error {
  fn from(e: jsonrpsee::core::Error) -> Self {
    Self::Jsonrpsee(e)
  }
}

impl From<sp_core::crypto::SecretStringError> for Error {
  fn from(e: sp_core::crypto::SecretStringError) -> Self {
    Self::SecretStringError(format!("{e:?}"))
  }
}

impl From<sp_core::crypto::PublicError> for Error {
  fn from(e: sp_core::crypto::PublicError) -> Self {
    Self::CoreCryptoError(format!("{e:?}"))
  }
}

impl From<subxt_signer::sr25519::Error> for Error {
  fn from(e: subxt_signer::sr25519::Error) -> Self {
    Self::SecretStringError(format!("{e:?}"))
  }
}

impl From<subxt_signer::ecdsa::Error> for Error {
  fn from(e: subxt_signer::ecdsa::Error) -> Self {
    Self::SecretStringError(format!("{e:?}"))
  }
}

impl From<subxt_signer::SecretUriError> for Error {
  fn from(e: subxt_signer::SecretUriError) -> Self {
    Self::SecretStringError(format!("{e:?}"))
  }
}

#[cfg(not(feature = "std"))]
impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

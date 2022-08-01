use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Std io error: {0}")]
  StdIo(#[from] std::io::Error),

  #[error("Json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("hex error: {0}")]
  Hex(#[from] hex::FromHexError),

  #[error("http error: {0}")]
  Http(#[from] http::Error),

  #[error("http uri error: {0}")]
  HttpUri(#[from] http::uri::InvalidUri),

  #[error("parity-scale-codec error: {0}")]
  ParityScaleCodec(#[from] codec::Error),

  #[error("Call API incompatible with connected chain: {0}")]
  IncompatibleCall(String),

  #[error("Schema failed to parse: {0}")]
  SchemaParseFailed(String),

  #[error("RpcClient: {0}")]
  RpcClient(String),

  #[error("Signing transaction failed: {0}")]
  SigningTransactionFailed(String),

  #[error("Jsonrpsee error: {0}")]
  Jsonrpsee(#[from] jsonrpsee::core::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

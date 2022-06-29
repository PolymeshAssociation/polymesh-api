use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Std io error: {0}")]
  StdIo(#[from] std::io::Error),

  #[error("Json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("hex error: {0}")]
  Hex(#[from] hex::FromHexError),

  #[error("parity-scale-codec error: {0}")]
  ParityScaleCodec(#[from] codec::Error),

  #[error("Schema failed to parse: {0}")]
  SchemaParseFailed(String),

  #[error("RpcClient: {0}")]
  RpcClient(String),

  #[cfg(feature = "rpc")]
  #[error("Jsonrpsee error: {0}")]
  Jsonrpsee(#[from] jsonrpsee::core::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

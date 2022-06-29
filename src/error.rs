use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Std io error: {0}")]
  StdIo(#[from] std::io::Error),

  #[error("Json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("Schema failed to parse: {0}")]
  SchemaParseFailed(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

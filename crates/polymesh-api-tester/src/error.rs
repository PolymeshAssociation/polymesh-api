use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Std io error: {0}")]
  StdIo(#[from] std::io::Error),

  #[error("Sqlx error: {0}")]
  Sqlx(#[from] sqlx::error::Error),

  #[error("Polymesh API Client error: {0}")]
  PolymeshApiClient(#[from] polymesh_api::client::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

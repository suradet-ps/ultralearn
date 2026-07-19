use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum AppError {
  #[error("{message}")]
  Config { message: String },
  #[error("{message}")]
  Http { status: u16, message: String },
  #[error("JSON error: {0}")]
  Json(String),
  #[error("Storage error: {0}")]
  Storage(String),
  #[error("Serialization error: {0}")]
  Serde(String),
  #[error("{0}")]
  Other(String),
}

impl AppError {
  pub fn config(m: impl Into<String>) -> Self {
    Self::Config { message: m.into() }
  }
  pub fn http(status: u16, m: impl Into<String>) -> Self {
    Self::Http {
      status,
      message: m.into(),
    }
  }
  pub fn storage(m: impl Into<String>) -> Self {
    Self::Storage(m.into())
  }
  pub fn serde(m: impl Into<String>) -> Self {
    Self::Serde(m.into())
  }
  pub fn other(m: impl Into<String>) -> Self {
    Self::Other(m.into())
  }
}

impl From<serde_json::Error> for AppError {
  fn from(e: serde_json::Error) -> Self {
    Self::Json(e.to_string())
  }
}

impl From<gloo_storage::errors::StorageError> for AppError {
  fn from(e: gloo_storage::errors::StorageError) -> Self {
    Self::Storage(e.to_string())
  }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LagoError {
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  #[error("Serialization error: {0}")]
  Serialization(#[from] serde_json::Error),

  #[error("API error: {status} - {message}")]
  Api { status: u16, message: String },

  #[error("Invalid configuration: {0}")]
  Configuration(String),

  #[error("Unauthorized: invalid API key")]
  Unauthorized,
}

pub type Result<T> = std::result::Result<T, LagoError>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("API error: {0}")]
  ApiError(u16),
  #[error("Network error: {0}")]
  NetworkError(#[from] reqwest::Error),
  #[error("Parsing error: {0}")]
  ParsingError(#[from] serde_json::Error),
  #[error("Error: {0}")]
  OtherError(String),
}
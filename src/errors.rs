//! Errors which may arise from this crate.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("message too large")]
    MessageTooLarge,
    #[error("Deserialize error")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

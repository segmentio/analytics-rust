//! Errors which may arise from this crate.

use thiserror::Error;

/// An enum of errors this crate may produce. These are compatible with
/// `failure` errors.
#[derive(Error, Debug)]
pub enum Error {
    /// The given message is too large to be sent to Segment's API.
    #[error("message too large")]
    MessageTooLarge,
    #[error("Deserialize error")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

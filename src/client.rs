//! Interfaces to the Segment tracking API.

use crate::message::Message;
use failure::Error;

/// `Client` is a trait representing the HTTP transport layer of the analytics library.
#[async_trait::async_trait]
pub trait Client {
    /// Send a single message to Segment using the given write key.
    ///
    /// A `write_key` is an API key for Segment's tracking API. See [Segment's
    /// documentation](https://segment.com/docs/guides/setup/how-do-i-find-my-write-key/)
    /// for how to find this value.
    async fn send(&self, write_key: String, msg: Message) -> Result<(), Error>;
}

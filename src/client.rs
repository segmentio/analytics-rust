//! Interfaces to the June tracking API.

use crate::message::Message;
use failure::Error;

/// `Client` is a trait representing the HTTP transport layer of the analytics library.
pub trait Client {
    /// Send a single message to June using the given write key.
    ///
    /// A `write_key` is an API key for June's tracking API.
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error>;
}

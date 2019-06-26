use crate::message::Message;
use failure::Error;

/// `Client` is a trait representing the HTTP transport layer of the analytics library.
pub trait Client {
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error>;
}

use crate::message::Message;
use failure::Error;

pub trait Client {
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error>;
}

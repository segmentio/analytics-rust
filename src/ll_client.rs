use crate::errors::Result;
use crate::message::Message;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn send(&self, msg: Message) -> Result<()> {
        Ok(())
    }
}

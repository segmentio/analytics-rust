use crate::buffer::Buffer;
use crate::errors::Result;
use crate::ll_client;
use crate::message::{Batch, Message};
use std::borrow::Cow;

pub struct StandardClientBuilder {}

impl StandardClientBuilder {
    pub fn build(self) -> Result<StandardClient> {
        Ok(StandardClient {
            ll_client: ll_client::Client::new(),
        })
    }
}

pub struct StandardClient {
    ll_client: ll_client::Client,
}

impl StandardClient {
    pub fn new_buffer<'a, S>(&'a self, write_key: S) -> Buffer<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Buffer::new(&self, write_key)
    }

    pub(crate) fn send(&self, batch: &Batch) -> Result<()> {
        self.ll_client.send(Message::Batch(batch))
    }

    //    pub(crate)
}

use crate::buffer::Buffer;
use crate::errors::Result;
use crate::ll_client;
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
    pub fn new_buffer<'a, S>(&'a self, write_key: &'a str) -> Buffer<'a> {
        Buffer::new(&self.ll_client, write_key)
    }
}

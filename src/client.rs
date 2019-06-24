use crate::batch::Batch;
use crate::errors::Result;
use crate::ll_client;
use std::borrow::Cow;

pub struct ClientBuilder {}

impl ClientBuilder {
    pub fn build(self) -> Result<Client> {
        Ok(Client {
            ll_client: ll_client::Client::new(),
        })
    }
}

pub struct Client {
    ll_client: ll_client::Client,
}

impl Client {
    pub fn new_batch<'a, S>(&self, write_key: &str) -> Batch {
        Batch::new(&self.ll_client, write_key)
    }
}

use crate::buffer::Buffer;
use crate::errors::Result;
use crate::ll_client;
use crate::message::{Batch, Message};
use std::borrow::Cow;
use std::time::Duration;
use url::Url;

pub struct StandardClientBuilder {
    url: Url,
    retryFn: Box<Fn(&StandardClient, &Batch) -> Result<()>>,
}

impl Default for StandardClientBuilder {
    fn default() -> Self {
        StandardClientBuilder {
            url: Url::parse("https://api.segment.io").unwrap(),
            retryFn: Box::new(|client, batch| {
                let mut result = Ok(());
                for _i in 0..5 {
                    result = client.ll_client.send(Message::Batch(batch));
                    if result.is_err() {
                        continue;
                    }
                }
                result
            }),
        }
    }
}

impl StandardClientBuilder {
    pub fn url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

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
}

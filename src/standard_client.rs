use crate::errors::Result;
use crate::ll_client;
use crate::message::{Batch, Message};
use std::borrow::Cow;
use std::thread;
use std::time::Duration;
use url::Url;

pub type RetryFn = Fn(&StandardClient, &str, &Batch) -> Result<()>;

pub struct StandardClientBuilder {
    url: Url,
    //    retry_fn: Box<RetryFn>,
}

impl Default for StandardClientBuilder {
    fn default() -> Self {
        StandardClientBuilder {
            url: Url::parse("https://api.segment.io").unwrap(),
            //            retry_fn: Box::new(|client, write_key, batch| {
            //                let mut result = Ok(());
            //                for _i in 0..5 {
            //                    let b = (*batch).clone();
            //                    result = client.ll_client.send(write_key, Message::Batch(b));
            //                    if result.is_err() {
            //                        thread::sleep(Duration::new(1, 0));
            //                        continue;
            //                    }
            //                }
            //                result
            //            }),
        }
    }
}

impl StandardClientBuilder {
    pub fn url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }

    //    pub fn retry_fn<F>(mut self, func: F) -> Self
    //    where
    //        F: Fn(&StandardClient, &str, &Batch) -> Result<()> + 'static,
    //    {
    //        self.retry_fn = Box::new(func);
    //        self
    //    }

    pub fn build(self) -> Result<StandardClient> {
        Ok(StandardClient {
            ll_client: ll_client::Client::new(),
            //            retry_fn: self.retry_fn,
        })
    }
}

pub struct StandardClient {
    ll_client: ll_client::Client,
    //    retry_fn: Box<RetryFn>,
}

impl StandardClient {
    //    pub fn new_buffer<'a, S>(&'a self, write_key: S) -> Buffer<'a>
    //    where
    //        S: Into<Cow<'a, str>>,
    //    {
    //        Buffer::new(&self, write_key)
    //    }
    //
    //    pub(crate) fn send(&self, write_key: &str, batch: &Batch) -> Result<()> {
    //        (self.retry_fn)(self, write_key, batch)
    //    }
}

use crate::errors::Result;
use crate::ll_client::Client;
use crate::message::{Batch, Message};
use std::borrow::Cow;
use uuid::Uuid;

pub struct Buffer<'a> {
    client: &'a Client,
    write_key: Cow<'a, str>,
    batch: Batch,
}

impl<'a> Buffer<'a> {
    pub(crate) fn new<S>(client: &'a Client, writekey: S) -> Buffer<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Buffer {
            client,
            write_key: writekey.into(),
            batch: Batch::new(Uuid::new_v4().to_string()),
        }
    }

    pub fn enqueue(&self, message: Message) -> Result<()> {
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        Ok(())
    }
}

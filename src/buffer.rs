use crate::errors::{Error, Result};
use crate::ll_client::Client;
use crate::message::{Batch, Message};
use std::borrow::Cow;
use uuid::Uuid;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

pub struct Buffer<'a> {
    client: &'a Client,
    write_key: Cow<'a, str>,
    batch: Batch,
    byte_count: usize,
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
            byte_count: 0,
        }
    }

    pub fn enqueue(&mut self, message: Message) -> Result<()> {
        let size = serde_json::to_vec(&message)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(Error::MessageTooLarge(String::from("message too large")));
        }
        self.byte_count += size;
        if self.byte_count > MAX_BATCH_SIZE {
            return Err(Error::MaxBatchSize(String::from(
                "maximum batch size reached",
            )));
        }
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        Ok(())
    }
}

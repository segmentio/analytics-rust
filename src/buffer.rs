use crate::errors::{Error, Result};
use crate::message::{Batch, BatchMessage, Message};
use crate::standard_client::StandardClient;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Buffer<'a> {
    client: &'a StandardClient,
    write_key: Cow<'a, str>,
    batch: Batch,
}

impl<'a> Buffer<'a> {
    pub(crate) fn new<S>(client: &'a StandardClient, writekey: S) -> Buffer<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Buffer {
            client,
            write_key: writekey.into(),
            batch: Batch::new(Uuid::new_v4().to_string()),
        }
    }

    pub fn enqueue(&mut self, message: BatchMessage) -> Result<()> {
        self.batch.add(message)
        //        let size = serde_json::to_vec(&message)?.len();
        //        if size > MAX_MESSAGE_SIZE {
        //            return Err(Error::MessageTooLarge(String::from("message too large")));
        //        }
        //        self.byte_count += size;
        //        if self.byte_count > MAX_BATCH_SIZE {
        //            return Err(Error::MaxBatchSize(String::from(
        //                "maximum batch size reached",
        //            )));
        //        }
        //        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        self.client.send(&self.batch)
    }
}

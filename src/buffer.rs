//use crate::errors::Result;
//use crate::message::{Batch, BatchMessage, Message};
//use crate::standard_client::StandardClient;
//use std::borrow::Cow;
//use uuid::Uuid;
//
//pub struct Buffer<'a> {
//    client: &'a StandardClient,
//    write_key: Cow<'a, str>,
//    batch: Batch,
//}
//
//impl<'a> Buffer<'a> {
//    pub(crate) fn new<S>(client: &'a StandardClient, writekey: S) -> Buffer<'a>
//    where
//        S: Into<Cow<'a, str>>,
//    {
//        Buffer {
//            client,
//            write_key: writekey.into(),
//            batch: Batch::new(Uuid::new_v4().to_string()),
//        }
//    }
//
//    pub fn enqueue(&mut self, message: BatchMessage) -> Result<()> {
//        self.batch.add(message)
//    }
//
//    pub fn flush(&self) -> Result<()> {
//        self.client.send(&self.write_key, &self.batch)
//    }
//}

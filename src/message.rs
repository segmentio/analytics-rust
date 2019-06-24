use crate::errors::{Error, Result};
use serde::Serialize;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

#[derive(Serialize)]
pub enum Message<'a> {
    Identify(&'a Identify),
    Track(&'a Track),
    Batch(&'a Batch),
}

#[derive(Serialize)]
pub struct Batch {
    message_id: String,
    messages: Vec<BatchMessage>,
    #[serde(skip_serializing)]
    byte_count: usize,
}

impl Batch {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            messages: Vec::new(),
            byte_count: 0,
        }
    }

    pub fn add(&mut self, message: BatchMessage) -> Result<()> {
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
        self.messages.push(message);
        Ok(())
    }

    pub fn reset(&mut self, message_id: String) {
        self.message_id = message_id;
        self.byte_count = 0;
        self.messages.clear();
    }
}

#[derive(Serialize)]
pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
}

#[derive(Serialize)]
pub struct Identify {}

#[derive(Serialize)]
pub struct Track {}

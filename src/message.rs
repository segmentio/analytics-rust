use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Group(Group),
    Screen(Screen),
    Alias(Alias),
    Batch(Batch),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Identify {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub event: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Screen {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "previousId")]
    pub previous_id: String,
}

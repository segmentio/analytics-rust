use serde::Serialize;

#[derive(Serialize)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Batch(Batch),
}

#[derive(Serialize)]
pub struct Batch {
    message_id: String,
    messages: Vec<BatchMessage>,
}

impl Batch {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            messages: Vec::new(),
        }
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

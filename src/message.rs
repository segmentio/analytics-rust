pub enum Message {
    Identify(Identify),
    Track(Track),
    Batch(Batch),
}

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

pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
}

pub struct Identify {}

pub struct Track {}

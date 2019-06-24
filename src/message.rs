pub enum Message {
    Identify(Identify),
    Track(Track),
    Batch(Batch),
}

pub struct Batch {
    messages: Vec<BatchMessage>,
}

pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
}

pub struct Identify {}

pub struct Track {}

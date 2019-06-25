use crate::errors::{Error, MaxBatchSize, Result};
use crate::message::{Batch, BatchMessage, Message};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use uuid::Uuid;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

pub struct Batcher {
    message_id: String,
    buf: Vec<BatchMessage>,
    byte_count: usize,
    context: Map<String, Value>,
}

impl Batcher {
    pub fn new(message_id: String, context: Map<String, Value>) -> Self {
        Self {
            message_id,
            buf: Vec::new(),
            byte_count: 0,
            context,
        }
    }

    /// if returns error, you message is garbo
    /// if returns some, this queue needs flushing
    /// if returns none, this message was accepted. it's mine now
    pub fn push(&mut self, msg: BatchMessage) -> Result<Option<Message>> {
        let size = serde_json::to_vec(&msg)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(Error::MessageTooLarge(String::from("message too large")));
        }
        self.byte_count += size + 1; // +1 to account for Serialized data's extra commas
        if self.byte_count > MAX_BATCH_SIZE {
            return Err(Error::MaxBatchSize(MaxBatchSize { message: msg }));
        }
        self.buf.push(msg);
        Ok(None)
    }

    pub fn into_message(self) -> Message {
        Message::Batch(Batch {
            message_id: self.message_id,
            messages: self.buf,
            sent_at: Utc::now(),
            context: self.context,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Track;

    #[test]
    fn test_push_and_into() {
        let batch_msg = Track {
            user_id: "id".to_string(),
            event: "login".to_string(),
        };

        let mut library = Map::new();
        library.insert("name".to_string(), "analytics-rust".into());
        library.insert("version".to_string(), env!("CARGO_PKG_VERSION").into());
        let mut context = Map::new();
        context.insert("library".to_string(), library.into());

        let mut batcher = Batcher::new("msg_id".to_string(), context.clone());
        let result = batcher.push(batch_msg.into());
        let batch = batcher.into_message();
        let inner_batch = match batch {
            Message::Batch(b) => b,
            _ => panic!("invalid message type"),
        };
        assert_eq!(context, inner_batch.context);
        assert_eq!(1, inner_batch.messages.len());

        let track = match inner_batch.messages.get(0).unwrap() {
            BatchMessage::Track(t) => t,
            _ => panic!("invalid message batch type"),
        };
        assert_eq!("id".to_string(), track.user_id);
        assert_eq!("login".to_string(), track.event);
    }

    #[test]
    fn test_bad_message_size() {
        let batch_msg = Track {
            user_id: String::from_utf8(vec![b'a'; 1024 * 33]).unwrap(), // 33KB message
            event: "login".to_string(),
        };
        let mut batcher = Batcher::new("msg_id".to_string(), Map::new());
        let result = batcher.push(batch_msg.into());
        assert_eq!(true, result.is_err());
        assert_eq!(
            true,
            match result.err() {
                Some(Error::MessageTooLarge(_)) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_max_buffer() {
        let batch_msg = Track {
            user_id: String::from_utf8(vec![b'a'; 1024 * 30]).unwrap(), // 3oKB message
            event: "login".to_string(),
        };
        let mut batcher = Batcher::new("msg_id".to_string(), Map::new());

        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone().into());
            if result.is_err() {
                break;
            }
        }
        assert_eq!(true, result.is_err());
        assert_eq!(
            Some(batch_msg),
            match result.err() {
                Some(Error::MaxBatchSize(s)) => match s.message {
                    BatchMessage::Track(t) => Some(t),
                    _ => None,
                },
                _ => None,
            }
        );
    }
}

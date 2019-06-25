use crate::errors::{Error as AnalyticsError, MaxBatchSize};
use crate::message::{Batch, BatchMessage, Context, Message};
use chrono::Utc;
use failure::Error;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

pub struct Batcher {
    message_id: String,
    buf: Vec<BatchMessage>,
    byte_count: usize,
    context: Option<Context>,
}

impl Batcher {
    pub fn new(message_id: String, context: Option<Context>) -> Self {
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
    pub fn push(&mut self, msg: BatchMessage) -> Result<Option<Message>, Error> {
        let size = serde_json::to_vec(&msg)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(AnalyticsError::MessageTooLarge("msg too large".to_owned()).into());
        }

        self.byte_count += size + 1; // +1 to account for Serialized data's extra commas
        if self.byte_count > MAX_BATCH_SIZE {
            return Err(AnalyticsError::MaxBatchSize(MaxBatchSize { message: msg }).into());
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
    use crate::message::{IdentifyingID, Library, Track};

    #[test]
    fn test_push_and_into() {
        let batch_msg = Track {
            id: Some(IdentifyingID::Id {
                id: "myid".to_owned(),
            }),
            event: "login".to_owned(),
            ..Default::default()
        };

        let context = Context {
            library: Some(Library {
                name: "analytics-rust".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut batcher = Batcher::new("msg_id".to_owned(), Some(context.clone()));
        let result = batcher.push(batch_msg.into());
        assert_eq!(None, result.ok().unwrap());

        let batch = batcher.into_message();
        let inner_batch = match batch {
            Message::Batch(b) => b,
            _ => panic!("invalid message type"),
        };
        assert_eq!(context, inner_batch.context.unwrap());
        assert_eq!(1, inner_batch.messages.len());

        let track = match inner_batch.messages.get(0).unwrap() {
            BatchMessage::Track(t) => t,
            _ => panic!("invalid message batch type"),
        };
        assert_eq!(
            &IdentifyingID::Id {
                id: "myid".to_owned()
            },
            track.id.as_ref().unwrap()
        );
        assert_eq!("login".to_owned(), track.event);
    }

    #[test]
    fn test_bad_message_size() {
        let batch_msg = Track {
            id: Some(IdentifyingID::Id {
                id: String::from_utf8(vec![b'a'; 1024 * 33]).unwrap(), // 33KB message
            }),
            event: "login".to_owned(),
            ..Default::default()
        };
        let mut batcher = Batcher::new("msg_id".to_owned(), None);
        let result = batcher.push(batch_msg.into());

        let err = result.err().unwrap();
        let err: &AnalyticsError = err.as_fail().downcast_ref().unwrap();

        match err {
            AnalyticsError::MessageTooLarge(_) => {}
            _ => panic!("wrong error type returned: {:?}", err),
        }
    }

    #[test]
    fn test_max_buffer() {
        let batch_msg = Track {
            id: Some(IdentifyingID::Id {
                id: String::from_utf8(vec![b'a'; 1024 * 30]).unwrap(), // 30KB message
            }),
            event: "login".to_owned(),
            ..Default::default()
        };
        let mut batcher = Batcher::new("msg_id".to_owned(), None);

        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone().into());
            if result.is_err() {
                break;
            }
        }

        let err = result.err().unwrap();
        let err: &AnalyticsError = err.as_fail().downcast_ref().unwrap();

        match err {
            AnalyticsError::MaxBatchSize(message) => {
                assert_eq!(message.message, BatchMessage::Track(batch_msg));
            }
            _ => panic!("wrong error type returned: {:?}", err),
        }
    }
}

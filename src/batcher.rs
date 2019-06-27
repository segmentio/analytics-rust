use crate::errors::Error as AnalyticsError;
use crate::message::{Batch, BatchMessage, Context, Message};
use chrono::Utc;
use failure::Error;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

// `Batcher` is a low level abstraction which helps creating a batch Message/Payload to be send to the APIs `/v1/batch` endpoint.
pub struct Batcher {
    message_id: String,
    buf: Vec<BatchMessage>,
    byte_count: usize,
    context: Option<Context>,
}

impl Batcher {
    /// `new` creates a `Batcher` for use.
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
    /// `push` attempts to add a new `BatchMessage` to the batch which results in one of the following outcomes:
    /// - Message is accepts and added to the batch
    /// - Message is rejected due to being invalid.
    /// - Message is accepted, but the batch has reached the maximum size the API supports. The message is then returned in the Result.
    pub fn push(&mut self, msg: BatchMessage) -> Result<Option<BatchMessage>, Error> {
        let size = serde_json::to_vec(&msg)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(AnalyticsError::MessageTooLarge("msg too large".to_owned()).into());
        }

        self.byte_count += size + 1; // +1 to account for Serialized data's extra commas
        if self.byte_count > MAX_BATCH_SIZE {
            return Ok(Some(msg));
        }

        self.buf.push(msg);
        Ok(None)
    }

    /// `into_message`
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
    use crate::message::{Library, Track, TrackBuilder};

    #[test]
    fn test_push_and_into() -> Result<(), Error> {
        let batch_msg = TrackBuilder::new("login")?.user_id("myid")?.build()?;
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
        assert_eq!(&"myid".to_owned(), &track.user_id.clone().unwrap());
        assert_eq!("login".to_owned(), track.event);
        Ok(())
    }

    #[test]
    fn test_bad_message_size() -> Result<(), Error> {
        let batch_msg = TrackBuilder::new("login")?
            .user_id(String::from_utf8(vec![b'a'; 1024 * 33]).unwrap())? // 33KB message
            .build()?;
        let mut batcher = Batcher::new("msg_id".to_owned(), None);
        let result = batcher.push(batch_msg.into());

        let err = result.err().unwrap();
        let err: &AnalyticsError = err.as_fail().downcast_ref().unwrap();

        match err {
            AnalyticsError::MessageTooLarge(_) => {}
            _ => panic!("wrong error type returned: {:?}", err),
        }
        Ok(())
    }

    #[test]
    fn test_max_buffer() -> Result<(), Error> {
        let batch_msg = TrackBuilder::new("login")?
            .user_id(String::from_utf8(vec![b'a'; 1024 * 30]).unwrap())? // 30KB message
            .build()?;
        let mut batcher = Batcher::new("msg_id".to_owned(), None);
        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone().into());
            dbg!(&result);
            if result.is_ok() && result.as_ref().ok().unwrap().is_some() {
                break;
            }
        }

        let msg = result.ok().unwrap();
        assert_eq!(BatchMessage::Track(batch_msg), msg.unwrap());
        Ok(())
    }
}

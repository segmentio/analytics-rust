use crate::message::{Batch, BatchMessage, Message};
use crate::{Error, Result};
use serde_json::{Map, Value};
use time::OffsetDateTime;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

/// A batcher can accept messages into an internal buffer, and report when
/// messages must be flushed.
///
/// The recommended usage pattern looks something like this:
///
/// ```
/// use june_analytics::{Batcher, Client, HttpClient};
/// use june_analytics::message::{BatchMessage, Track, User};
/// use serde_json::json;
///
/// let mut batcher = Batcher::new(None);
/// let client = HttpClient::default();
///
/// for i in 0..100 {
///     let msg = Track {
///         user: User::UserId { user_id: format!("user-{}", i) },
///         event: "Example".to_owned(),
///         properties: json!({ "foo": "bar" }),
///         ..Default::default()
///     };
///
///     // Batcher returns back ownership of a message if the internal buffer
///     // would overflow.
///     //
///     // When this occurs, we flush the batcher, create a new batcher, and add
///     // the message into the new batcher.
///     if let Some(msg) = batcher.push(msg).unwrap() {
///         client.send("your_write_key".to_string(), batcher.into_message());
///         batcher = Batcher::new(None);
///         batcher.push(msg).unwrap();
///     }
/// }
/// ```
///
/// Batcher will attempt to fit messages into maximally-sized batches, thus
/// reducing the number of round trips required with June's tracking API.
/// However, if you produce messages infrequently, this may significantly delay
/// the sending of messages to June.
///
/// If this delay is a concern, it is recommended that you periodically flush
/// the batcher on your own by calling `into_message`.
///
/// By default if the message you push in the batcher does not contains any
/// timestamp, the timestamp at the time of the push will be automatically
/// added to your message.
/// You can disable this behaviour with the [without_auto_timestamp] method
/// though.
#[derive(Clone, Debug)]
pub struct Batcher {
    pub(crate) buf: Vec<BatchMessage>,
    pub(crate) byte_count: usize,
    pub(crate) context: Option<Value>,
    pub(crate) auto_timestamp: bool,
}

impl Batcher {
    /// Construct a new, empty batcher.
    ///
    /// Optionally, you may specify a `context` that should be set on every
    /// batch returned by `into_message`.
    pub fn new(context: Option<Value>) -> Self {
        Self {
            buf: Vec::new(),
            byte_count: 0,
            context,
            auto_timestamp: true,
        }
    }

    pub fn without_auto_timestamp(&mut self) {
        self.auto_timestamp = false;
    }

    /// Push a message into the batcher.
    ///
    /// Returns `Ok(None)` if the message was accepted and is now owned by the
    /// batcher.
    ///
    /// Returns `Ok(Some(msg))` if the message was rejected because the current
    /// batch would be oversized if this message were accepted. The given
    /// message is returned back, and it is recommended that you flush the
    /// current batch before attempting to push `msg` in again.
    ///
    /// Returns an error if the message is too large to be sent to June's
    /// API.
    pub fn push(&mut self, msg: impl Into<BatchMessage>) -> Result<Option<BatchMessage>> {
        let mut msg: BatchMessage = msg.into();
        let timestamp = msg.timestamp_mut();
        if self.auto_timestamp && timestamp.is_none() {
            *timestamp = Some(OffsetDateTime::now_utc());
        }
        let size = serde_json::to_vec(&msg)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(Error::MessageTooLarge);
        }

        self.byte_count += size + 1; // +1 to account for Serialized data's extra commas
        if self.byte_count > MAX_BATCH_SIZE {
            return Ok(Some(msg));
        }

        self.buf.push(msg);
        Ok(None)
    }

    pub fn into_message(self) -> Message {
        Message::Batch(Batch {
            batch: self.buf,
            context: self.context,
            integrations: None,
            extra: Map::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{Track, User};
    use serde_json::json;

    #[test]
    fn test_push_and_into() {
        let batch_msg = BatchMessage::Track(Track {
            ..Default::default()
        });

        let context = json!({
            "foo": "bar",
        });

        let mut batcher = Batcher::new(Some(context.clone()));
        batcher.without_auto_timestamp();
        let result = batcher.push(batch_msg.clone());
        assert_eq!(None, result.ok().unwrap());

        let batch = batcher.into_message();
        let inner_batch = match batch {
            Message::Batch(b) => b,
            _ => panic!("invalid message type"),
        };
        assert_eq!(context, inner_batch.context.unwrap());
        assert_eq!(1, inner_batch.batch.len());

        assert_eq!(inner_batch.batch, vec![batch_msg]);
    }

    #[test]
    fn test_bad_message_size() {
        let batch_msg = Track {
            user: User::UserId {
                user_id: String::from_utf8(vec![b'a'; 1024 * 33]).unwrap(),
            },
            ..Default::default()
        };

        let mut batcher = Batcher::new(None);
        let result = batcher.push(batch_msg);

        let err = result.err().unwrap();
        assert!(err.to_string().contains("message too large"));
    }

    #[test]
    fn test_max_buffer() {
        let batch_msg = Track {
            user: User::UserId {
                user_id: String::from_utf8(vec![b'a'; 1024 * 30]).unwrap(),
            },
            ..Default::default()
        };

        let mut batcher = Batcher::new(None);
        batcher.without_auto_timestamp();
        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone());
            dbg!(&result);
            if result.is_ok() && result.as_ref().ok().unwrap().is_some() {
                break;
            }
        }

        let msg = result.ok().unwrap();
        assert_eq!(BatchMessage::from(batch_msg), msg.unwrap());
    }
}

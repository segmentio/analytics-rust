//! Utilities for batching up messages.

use crate::errors::Error as AnalyticsError;
use crate::message::{Batch, BatchMessage, Message};
use failure::Error;
use serde_json::{Map, Value};

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

/// A batcher can accept messages into an internal buffer, and report when
/// messages must be flushed.
///
/// The recommended usage pattern looks something like this:
///
/// ```
/// use analytics::batcher::Batcher;
/// use analytics::client::Client;
/// use analytics::http::HttpClient;
/// use analytics::message::{BatchMessage, Track, User};
/// use serde_json::json;
///
/// let mut batcher = Batcher::new(None);
/// let client = HttpClient::default();
///
/// for i in 0..100 {
///     let msg = BatchMessage::Track(Track {
///         user: User::UserId { user_id: format!("user-{}", i) },
///         event: "Example".to_owned(),
///         properties: json!({ "foo": "bar" }),
///         ..Default::default()
///     });
///
///     // Batcher returns back ownership of a message if the internal buffer
///     // would overflow.
///     //
///     // When this occurs, we flush the batcher, create a new batcher, and add
///     // the message into the new batcher.
///     if let Some(msg) = batcher.push(msg).unwrap() {
///         client.send("your_write_key", &batcher.into_message()).unwrap();
///         batcher = Batcher::new(None);
///         batcher.push(msg).unwrap();
///     }
/// }
/// ```
///
///
/// If this delay is a concern, it is recommended that you periodically flush
/// the batcher on your own by calling `into_message`.
pub struct Batcher {
    buf: Vec<BatchMessage>,
    byte_count: usize,
    context: Option<Value>,
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
        }
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
    pub fn push(&mut self, msg: BatchMessage) -> Result<Option<BatchMessage>, Error> {
        let size = serde_json::to_vec(&msg)?.len();
        if size > MAX_MESSAGE_SIZE {
            return Err(AnalyticsError::MessageTooLarge.into());
        }

        self.byte_count += size + 1; // +1 to account for Serialized data's extra commas
        if self.byte_count > MAX_BATCH_SIZE {
            return Ok(Some(msg));
        }

        self.buf.push(msg);
        Ok(None)
    }

    /// Consumes this batcher and converts it into a message that can be sent
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
        let batch_msg = BatchMessage::Track(Track {
            user: User::UserId {
                user_id: String::from_utf8(vec![b'a'; 1024 * 33]).unwrap(),
            },
            ..Default::default()
        });

        let mut batcher = Batcher::new(None);
        let result = batcher.push(batch_msg.into());

        let err = result.err().unwrap();
        let err: &AnalyticsError = err.as_fail().downcast_ref().unwrap();

        match err {
            AnalyticsError::MessageTooLarge => {}
        }
    }

    #[test]
    fn test_max_buffer() {
        let batch_msg = BatchMessage::Track(Track {
            user: User::UserId {
                user_id: String::from_utf8(vec![b'a'; 1024 * 30]).unwrap(),
            },
            ..Default::default()
        });

        let mut batcher = Batcher::new(None);
        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone().into());
            dbg!(&result);
            if result.is_ok() && result.as_ref().ok().unwrap().is_some() {
                break;
            }
        }

        let msg = result.ok().unwrap();
        assert_eq!(batch_msg, msg.unwrap());
    }
}

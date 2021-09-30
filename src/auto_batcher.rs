//! Utilities for batching up messages.
//! When a batch is full it is automatically sent over the network

use serde_json::Map;

use crate::{
    batcher::Batcher,
    client::Client,
    errors::Result,
    http::HttpClient,
    message::{Batch, BatchMessage, Message},
};

/// A batcher can accept messages into an internal buffer, and report when
/// messages must be flushed.
///
/// The recommended usage pattern looks something like this:
///
/// ```
/// use segment::AutoBatcher;
/// use segment::batcher::Batcher;
/// use segment::http::HttpClient;
/// use segment::message::{BatchMessage, Track, User};
/// use serde_json::json;
///
/// let client = HttpClient::default();
/// let batcher= Batcher::new(None);
/// let mut batcher = AutoBatcher::new(client, batcher, "your_write_key".to_string());
///
/// for i in 0..100 {
///     let msg = BatchMessage::Track(Track {
///         user: User::UserId { user_id: format!("user-{}", i) },
///         event: "Example".to_owned(),
///         properties: json!({ "foo": "bar" }),
///         ..Default::default()
///     });
///
///     batcher.push(msg); // .await
/// }
/// ```
///
/// Batcher will attempt to fit messages into maximally-sized batches, thus
/// reducing the number of round trips required with Segment's tracking API.
/// However, if you produce messages infrequently, this may significantly delay
/// the sending of messages to Segment.
///
/// If this delay is a concern, it is recommended that you periodically flush
/// the batcher on your own by calling [Self::flush].
pub struct AutoBatcher {
    client: HttpClient,
    batcher: Batcher,
    key: String,
}

impl AutoBatcher {
    /// Construct a new, empty batcher.
    ///
    /// ```
    /// use segment::AutoBatcher;
    /// use segment::batcher::Batcher;
    /// use segment::http::HttpClient;
    ///
    /// let client = HttpClient::default();
    /// let batcher= Batcher::new(None);
    /// let mut batcher = AutoBatcher::new(client, batcher, "your_write_key".to_string());
    /// ```
    pub fn new(client: HttpClient, batcher: Batcher, key: String) -> Self {
        Self {
            batcher,
            client,
            key,
        }
    }

    /// Push a message into the batcher.
    /// If the batcher is full, send it and create a new batcher with the message.
    ///
    /// Returns an error if the message is too large to be sent to Segment's
    /// API.
    ///
    /// ```
    /// use segment::AutoBatcher;
    /// use segment::batcher::Batcher;
    /// use segment::http::HttpClient;
    ///
    /// let client = HttpClient::default();
    /// let batcher= Batcher::new(None);
    /// let mut batcher = AutoBatcher::new(client, batcher, "your_write_key".to_string());
    ///
    /// let msg = BatchMessage::Track(Track {
    ///     user: User::UserId { user_id: format!("user-{}", i) },
    ///     event: "Example".to_owned(),
    ///     properties: json!({ "foo": "bar" }),
    ///     ..Default::default()
    /// });
    ///
    /// batcher.push(msg); // .await
    /// ```
    pub async fn push(&mut self, msg: BatchMessage) -> Result<()> {
        if let Some(msg) = self.batcher.push(msg)? {
            self.flush().await?;
            // this can't return None: the batcher is empty and if the message is
            // larger than the max size of the batcher it's supposed to throw an error
            self.batcher.push(msg)?;
        }

        Ok(())
    }

    /// Send all the message currently contained in the batcher, full or empty.
    ///
    /// Returns an error if the message is too large to be sent to Segment's
    /// API.
    /// ```
    /// use segment::AutoBatcher;
    /// use segment::batcher::Batcher;
    /// use segment::http::HttpClient;
    ///
    /// let client = HttpClient::default();
    /// let batcher= Batcher::new(None);
    /// let mut batcher = AutoBatcher::new(client, batcher, "your_write_key".to_string());
    ///
    /// let msg = BatchMessage::Track(Track {
    ///     user: User::UserId { user_id: format!("user-{}", i) },
    ///     event: "Example".to_owned(),
    ///     properties: json!({ "foo": "bar" }),
    ///     ..Default::default()
    /// });
    ///
    /// batcher.push(msg); // .await
    /// batcher.flush(); // .await
    /// ```
    pub async fn flush(&mut self) -> Result<()> {
        let message = Message::Batch(Batch {
            batch: std::mem::take(&mut self.batcher.buf),
            context: self.batcher.context.clone(),
            integrations: None,
            extra: Map::default(),
        });

        self.client.send(self.key.to_string(), message).await?;
        Ok(())
    }
}

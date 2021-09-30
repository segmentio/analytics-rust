//! A Rust client to Segment's tracking API.
//!
//! For more on what Segment is, refer to [Segment's
//! documentation](https://segment.com/docs/).
//!
//! ## Examples
//!
//! ### Simple
//! ```rust
//! use segment::http::HttpClient;
//! use segment::client::Client;
//! use segment::message::{Track, Message, User};
//! use serde_json::json;
//!
//! fn main() {
//!     let write_key = "YOUR_WRITE_KEY";
//!
//!     let client = HttpClient::default();
//!     let _ = client.send(write_key.to_string(), Message::Track(Track {
//!         user: User::UserId { user_id: "some_user_id".to_owned() },
//!         event: "Example Event".to_owned(),
//!         properties: json!({
//!             "some property": "some value",
//!             "some other property": "some other value",
//!         }),
//!         ..Default::default()
//!     }));
//! }
//! ```
//!
//! ### ETL-Like
//! ```rust
//! use segment::http::HttpClient;
//! use segment::client::Client;
//! use segment::message::{BatchMessage, Track, User};
//! use segment::batcher::Batcher;
//! use serde_json::json;
//!
//! fn main() {
//!     let write_key = "YOUR_WRITE_KEY";
//!
//!     let client = HttpClient::default();
//!     let mut batcher = Batcher::new(None);
//!
//!     // Pretend this is reading off of a queue, a file, or some other data
//!     // source.
//!     for i in 0..100 {
//!         let msg = BatchMessage::Track(Track {
//!             user: User::UserId { user_id: format!("user-{}", i) },
//!             event: "Example Event".to_owned(),
//!             properties: json!({
//!                 "foo": format!("bar-{}", i),
//!             }),
//!             ..Default::default()
//!         });
//!
//!         // An error here indicates a message is too large. In real life, you
//!         // would probably want to put this message in a deadletter queue or some
//!         // equivalent.
//!         if let Some(msg) = batcher.push(msg).unwrap() {
//!             let _ = client.send(write_key.to_string(), batcher.into_message());
//!
//!             batcher = Batcher::new(None);
//!             batcher.push(msg).unwrap(); // Same error condition as above.
//!         }
//!     }
//! }
//! ```

mod auto_batcher;
pub mod batcher;
pub mod client;
pub mod errors;
pub mod http;
pub mod message;

pub use auto_batcher::AutoBatcher;

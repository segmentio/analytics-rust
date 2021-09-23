//! An example showing how to do an ETL-like operation loading events into
//! Segment.

use segment::batcher::Batcher;
use segment::client::Client;
use segment::http::HttpClient;
use segment::message::{BatchMessage, Track, User};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    let mut batcher = Batcher::new(None);

    // Pretend this is reading off of a queue, a file, or some other data
    // source.
    for i in 0..100 {
        let msg = BatchMessage::Track(Track {
            user: User::UserId {
                user_id: format!("user-{}", i),
            },
            event: "Batched Event".to_owned(),
            properties: json!({
                "foo": format!("bar-{}", i),
            }),
            ..Default::default()
        });

        // An error here indicates a message is too large. In real life, you
        // would probably want to put this message in a deadletter queue or some
        // equivalent.
        if let Some(msg) = batcher.push(msg).unwrap() {
            client
                .send(write_key.to_string(), batcher.into_message())
                .await
                .unwrap();

            batcher = Batcher::new(None);
            batcher.push(msg).unwrap(); // Same error condition as above.
        }
    }

    client
        .send(write_key.to_string(), batcher.into_message())
        .await
        .unwrap();
}

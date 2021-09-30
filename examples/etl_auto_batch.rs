//! An example showing how to do an ETL-like operation loading events into
//! Segment using the `AutoBatcher`.

use segment::batcher::Batcher;
use segment::http::HttpClient;
use segment::message::{BatchMessage, Track, User};
use segment::AutoBatcher;
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    let batcher = Batcher::new(None);
    let mut batcher = AutoBatcher::new(client, batcher, write_key.to_string());

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

        batcher.push(msg).await.unwrap();
    }

    batcher.flush().await.unwrap();
}

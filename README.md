# Analytics Rust &emsp; [![Build Status]][circleci] [![Latest Version]][crates.io]


[Build Status]: https://circleci.com/gh/segmentio/analytics-rust.svg?style=badge
[circleci]: https://circleci.com/gh/segmentio/analytics-rust
[Latest Version]: https://img.shields.io/crates/v/analytics.svg
[crates.io]: https://crates.io/crates/analytics

**Analytics Rust is a Segment analytics client for Rust. For additional documentation visit https://segment.com/docs/sources/#server.**

---

```toml
[dependencies]
analytics = "0.2"
```

## Example usage(s)
```rust
use analytics::http::HttpClient;
use analytics::client::Client;
use analytics::message::{BatchMessage, Track, User};
use analytics::batcher::Batcher;
use serde_json::json;

fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    let mut batcher = Batcher::new(None);

    // Pretend this is reading off of a queue, a file, or some other data
    // source.
    for i in 0..100 {
        let msg = BatchMessage::Track(Track {
            user: User::UserId { user_id: format!("user-{}", i) },
            event: "Example Event".to_owned(),
            properties: json!({
                "foo": format!("bar-{}", i),
            }),
            ..Default::default()
        });

        // An error here indicates a message is too large. In real life, you
        // would probably want to put this message in a deadletter queue or some
        // equivalent.
        if let Some(msg) = batcher.push(msg).unwrap() {
            client.send(write_key, &batcher.into_message()).unwrap();

            batcher = Batcher::new(None);
            batcher.push(msg).unwrap(); // Same error condition as above.
        }
    }
}
```

or when you want to do struct to struct transformations

```rust
use analytics::http::HttpClient;
use analytics::client::Client;
use analytics::message::{Track, Message, User};
use serde_json::json;

fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    client.send(write_key, &Message::Track(Track {
        user: User::UserId { user_id: "some_user_id".to_owned() },
        event: "Example Event".to_owned(),
        properties: json!({
            "some property": "some value",
            "some other property": "some other value",
        }),
        ..Default::default()
    })).expect("could not send to Segment");
}

```

#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

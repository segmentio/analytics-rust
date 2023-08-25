# June Analytics Rust &emsp; [![Latest Version]][crates.io]


[Latest Version]: https://img.shields.io/crates/v/june-analytics.svg
[crates.io]: https://crates.io/crates/june-analytics

**Analytics Rust is a June analytics client for Rust.**

- Mostly from https://github.com/meilisearch/segment

---

```toml
[dependencies]
june-analytics = "0.2"
```

## Example usage(s)
```rust
use june_analytics::{HttpClient, Client, AutoBatcher, Batcher};
use june_analytics::message::{Track, User};
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
        let msg = Track {
            user: User::UserId { user_id: format!("user-{}", i) },
            event: "Example Event".to_owned(),
            properties: json!({
                "foo": format!("bar-{}", i),
            }),
            ..Default::default()
        };

        // An error here indicates a message is too large. In real life, you
        // would probably want to put this message in a deadletter queue or some
        // equivalent.
        batcher.push(msg).await.unwrap();
    }

    batcher.flush().await.unwrap();
}
```

or when you want to do struct to struct transformations

```rust
use june_analytics::{HttpClient, Client};
use june_analytics::message::{Track, Message, User};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    client.send(write_key.to_string(), Message::from(Track {
        user: User::UserId { user_id: "some_user_id".to_owned() },
        event: "Example Event".to_owned(),
        properties: json!({
            "some property": "some value",
            "some other property": "some other value",
        }),
        ..Default::default()
    })).await.expect("could not send to June");
}

```

## License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

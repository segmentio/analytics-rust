use analytics::client::Client;
use analytics::http::HttpClient;
use analytics::message::{Identify, Message};
use failure::Error;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let req = reqwest::Client::builder()
        .connect_timeout(Some(Duration::new(10, 0)))
        .build()?;
    let client = HttpClient::new(req, "https://api.segment.io".to_owned());
    let resp = client.send(
        "segment_write_key",
        &Message::Identify(Identify {
            message_id: "my unique id".to_owned(),
            anonymous_id: Some("anano".to_owned()),
            user_id: None,
            context: None,
            integrations: None,
            timestamp: None,
            traits: None,
        }),
    )?;
    if resp.status().as_u16() < 300 {
        println!("success");
    } else {
        println!("failure");
    }
    Ok(())
}

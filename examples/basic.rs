use analytics::client::Client;
use analytics::http::HttpClient;
use analytics::message::{Identify, Message};
use failure::Error;

fn main() -> Result<(), Error> {
    let client = HttpClient::default();
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
    );
    match resp {
        Ok(_) => println!("success"),
        Err(e) => println!("failure: {}", e),
    };
    Ok(())
}

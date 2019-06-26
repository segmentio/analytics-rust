use analytics::batcher::Batcher;
use analytics::client::Client;
use analytics::http::HttpClient;
use analytics::message::{BatchMessage, IdentifyBuilder};
use failure::Error;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let req = reqwest::Client::builder()
        .connect_timeout(Some(Duration::new(10, 0)))
        .build()?;
    let client = HttpClient::new(req, "https://api.segment.io".to_owned());

    let mut batcher = Batcher::new("my unique messzge id".to_owned(), None);

    // assume we have a message coming off of a queue or a bunch of messages we want to send in a batch
    for _i in 0..10 {
        let resp = batcher.push(BatchMessage::Identify(
            IdentifyBuilder::new()?
                .message_id("my unique user id")?
                .build()?,
        ));
        match resp {
            Ok(v) => match v {
                None => continue,
                Some(msg) => {
                    let payload = batcher.into_message();
                    batcher = Batcher::new("my unique messzge id".to_owned(), None);
                    batcher.push(msg)?;
                    match client.send("segment_write_key", &payload) {
                        Ok(_) => println!("successfully published batch"),
                        Err(e) => println!("failed to send payload: {}", e),
                    }
                }
            },
            Err(e) => {
                println!("error:{}", e);
            }
        };
    }
    Ok(())
}

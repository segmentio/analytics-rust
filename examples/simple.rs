use analytics::batcher::Batcher;
use analytics::client::Client;
use analytics::errors::Error as AnalyticsError;
use analytics::http::HttpClient;
use analytics::message::{BatchMessage, IdentifyBuilder};
use failure::Error;
use std::borrow::Cow;
use uuid::Uuid;

fn main() -> Result<(), Error> {
    let client = HttpClient::default();
    let mut batcher = BatchControl::new("segment_write_key", &client);

    // assume we have a message coming off of a queue or a bunch of messages we want to send in a batch
    for _i in 0..10 {
        let msg = IdentifyBuilder::new()?.user_id("unique user id")?.build()?;

        match batcher.push(msg) {
            Ok(_) => {
                println!("message accepted");
                continue;
            }
            Err(e) => match e.as_fail().downcast_ref::<AnalyticsError>() {
                Some(e) => match e {
                    AnalyticsError::MessageTooLarge(e) => {
                        println!("uh oh, message is invalid do something!: {}", e);
                    }
                    _ => {}
                },
                None => println!("error batching message:{}", e),
            },
        };
    }
    batcher.flush()?;
    println!("successfully published to the api! :)");
    Ok(())
}

struct BatchControl<'a, C> {
    client: &'a C,
    write_key: Cow<'a, str>,
    batcher: Batcher,
}

impl<'a, C: Client> BatchControl<'a, C> {
    pub fn new<S>(write_key: S, client: &'a C) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            client,
            write_key: write_key.into(),
            batcher: Batcher::new(Uuid::new_v4().to_string(), None),
        }
    }

    pub fn push<M>(&mut self, msg: M) -> Result<(), Error>
    where
        M: Into<BatchMessage>,
    {
        if let Some(msg) = self.batcher.push(msg.into())? {
            self.flush()?;
            self.batcher.push(msg)?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        // implement retries here
        self.client.send(
            &self.write_key,
            &std::mem::replace(
                &mut self.batcher,
                Batcher::new(Uuid::new_v4().to_string(), None),
            )
            .into_message(),
        )
    }
}

//! Low-level HTTP bindings to the June tracking API.

use crate::client::Client;
use crate::message::Message;
use failure::Error;
use std::time::Duration;

/// A client which synchronously sends single messages to the June tracking
/// API.
///
/// `HttpClient` implements [`Client`](../client/trait.Client.html); see the
/// documentation for `Client` for more on how to send events to June.
pub struct HttpClient {
    client: reqwest::blocking::Client,
    host: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .unwrap(),
            host: "https://api.june.so".to_owned(),
        }
    }
}

impl HttpClient {
    /// Construct a new `HttpClient` from a `reqwest::Client` and a June API
    /// scheme and host.
    ///
    /// If you don't care to re-use an existing `reqwest::Client`, you can use
    /// the `Default::default` value, which will send events to
    /// `https://api.june.so`.
    pub fn new(client: reqwest::blocking::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}

impl Client for HttpClient {
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error> {
        let path = match msg {
            Message::Identify(_) => "/sdk/identify",
            Message::Track(_) => "/sdk/track",
            Message::Page(_) => "/sdk/page",
            Message::Screen(_) => "/sdk/screen",
            Message::Group(_) => "/sdk/group",
            Message::Alias(_) => "/sdk/alias",
            Message::Batch(_) => "/sdk/batch",
        };

        self.client
            .post(&format!("{}{}", self.host, path))
            .basic_auth(write_key, Some(""))
            .json(msg)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}

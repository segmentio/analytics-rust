//! Low-level HTTP bindings to the Segment tracking API.

use crate::client::Client;
use crate::message::Message;
use failure::Error;
use std::time::Duration;

/// A client which synchronously sends single messages to the Segment tracking
/// API.
///
/// `HttpClient` implements [`Client`](../client/trait.Client.html); see the
/// documentation for `Client` for more on how to send events to Segment.
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
            host: "https://api.segment.io".to_owned(),
        }
    }
}

impl HttpClient {
    /// Construct a new `HttpClient` from a `reqwest::Client` and a Segment API
    /// scheme and host.
    ///
    /// If you don't care to re-use an existing `reqwest::Client`, you can use
    /// the `Default::default` value, which will send events to
    /// `https://api.segment.io`.
    pub fn new(client: reqwest::blocking::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}

impl Client for HttpClient {
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error> {
        let path = match msg {
            Message::Identify(_) => "/v1/identify",
            Message::Track(_) => "/v1/track",
            Message::Page(_) => "/v1/page",
            Message::Screen(_) => "/v1/screen",
            Message::Group(_) => "/v1/group",
            Message::Alias(_) => "/v1/alias",
            Message::Batch(_) => "/v1/batch",
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

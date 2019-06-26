use crate::client::Client;
use crate::message::Message;
use failure::{format_err, Error};
use reqwest::StatusCode;
use std::time::Duration;

pub struct HttpClient {
    client: reqwest::Client,
    host: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: reqwest::Client::builder()
                .connect_timeout(Some(Duration::new(10, 0)))
                .build()
                .unwrap(),
            host: "https://api.segment.io".to_owned(),
        }
    }
}

impl HttpClient {
    pub fn new(client: reqwest::Client, host: String) -> HttpClient {
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

        let resp = self
            .client
            .post(&format!("{}{}", self.host, path))
            .basic_auth(write_key, Some(""))
            .json(msg)
            .send()?;

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(format_err!(
                "http request failed: {}",
                StatusCode::BAD_REQUEST
            )),
            StatusCode::PAYLOAD_TOO_LARGE => Err(format_err!(
                "http request failed: {}",
                StatusCode::PAYLOAD_TOO_LARGE
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(format_err!(
                "http request failed: {}",
                StatusCode::INTERNAL_SERVER_ERROR
            )),
            s => Err(format_err!(
                "http request failed, unrecognized response: {}",
                s
            )),
        }
    }
}

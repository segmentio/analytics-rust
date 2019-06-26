use crate::client::Client;
use crate::message::Message;
use failure::Error;

pub struct HttpClient {
    client: reqwest::Client,
    host: String,
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

        self.client
            .post(&format!("{}{}", self.host, path))
            .basic_auth(write_key, Some(""))
            .json(msg)
            .send()?;

        Ok(())
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new(reqwest::Client::new(), "https://api.segment.io".to_owned())
    }
}

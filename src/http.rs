use crate::client::Client;
use crate::message::Message;

pub struct HttpClient {
    client: reqwest::Client,
    host: String,
}

impl HttpClient {
    pub fn new(client: reqwest::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}

impl Client<reqwest::Response, reqwest::Error> for HttpClient {
    fn send(&self, write_key: &str, msg: &Message) -> Result<reqwest::Response, reqwest::Error> {
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
            .send()
    }
}

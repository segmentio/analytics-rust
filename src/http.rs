use crate::Client;
use crate::Message;
use crate::Result;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct HttpClient {
    client: reqwest::Client,
    host: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: reqwest::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .unwrap(),
            host: "https://api.june.so".to_owned(),
        }
    }
}

impl HttpClient {
    pub fn new(client: reqwest::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}

#[async_trait::async_trait]
impl Client for HttpClient {
    async fn send(&self, write_key: String, msg: Message) -> Result<()> {
        let path = match msg {
            Message::Identify(_) => "/sdk/identify",
            Message::Track(_) => "/sdk/track",
            Message::Page(_) => "/sdk/page",
            Message::Screen(_) => "/sdk/screen",
            Message::Group(_) => "/sdk/group",
            Message::Alias(_) => "/sdk/alias",
            Message::Batch(_) => "/sdk/batch",
        };

        let _ = self
            .client
            .post(&format!("{}{}", self.host, path))
            .basic_auth(write_key, Some(""))
            .json(&msg)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
    Batch(Batch),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Batch {
    pub messages: Vec<BatchMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identify {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub event: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Screen {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "previousId")]
    pub previous_id: String,
}

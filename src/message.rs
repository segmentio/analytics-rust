use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Group(Group),
    Screen(Screen),
    Alias(Alias),
    Batch(Batch),
}

// TODO: add context, serde field serialize+deserialize renames
#[derive(Debug, Deserialize, Serialize)]
pub struct Batch {
    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "batch")]
    pub messages: Vec<BatchMessage>,

    #[serde(rename = "sentAt")]
    pub sent_at: DateTime<Utc>,

    #[serde(rename = "context")]
    pub context: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
}

macro_rules! msg_impl {
    ($id:ident) => {
        impl From<$id> for Message {
            fn from(msg: $id) -> Self {
                Message::$id(msg)
            }
        }

        impl From<$id> for BatchMessage {
            fn from(msg: $id) -> Self {
                BatchMessage::$id(msg)
            }
        }
    };
}

msg_impl!(Identify);
msg_impl!(Track);
msg_impl!(Page);
msg_impl!(Screen);
msg_impl!(Group);
msg_impl!(Alias);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct App {
    name: String,
    version: String,
    build: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Campaign {
    name: String,
    source: String,
    medium: String,
    term: String,
    content: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Device {
    id: String,
    manufacturer: String,
    model: String,
    name: String,

    #[serde(rename = "type")]
    device_type: String,

    version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Library {
    name: String,
    version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Location {
    city: String,
    country: String,
    latitude: i32,
    longitude: i32,
    region: String,
    speed: u32,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Network {
    bluetooth: bool,
    carrier: String,
    cellular: bool,
    wifi: bool,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Os {
    name: String,
    version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Webpage {
    hash: String,
    path: String,
    referrer: String,
    search: String,
    title: String,
    url: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Referrer {
    #[serde(rename = "type")]
    referrer_type: String,
    name: String,
    url: String,
    link: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeviceScreen {
    #[serde(rename = "type")]
    density: u32,
    height: u32,
    width: u32,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Context {
    /// Whether a user is active
    ///
    /// This is usually used to flag an .identify() call to just update the traits but not “last seen.”
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<App>,

    #[serde(skip_serializing_if = "Option::is_none")]
    campaign: Option<Campaign>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    library: Option<Library>,

    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<Os>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<Webpage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    referrer: Option<Referrer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen: Option<DeviceScreen>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identify {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    anonymous_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Track {
    #[serde(rename = "userId")]
    pub user_id: String,

    pub event: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Page {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Screen {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Alias {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "previousId")]
    pub previous_id: String,
}

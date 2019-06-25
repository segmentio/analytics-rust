use chrono::{Date, DateTime, Utc};
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

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct App {
    name: String,
    version: String,
    build: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Campaign {
    name: String,
    source: String,
    medium: String,
    term: String,
    content: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Library {
    name: String,
    version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Location {
    city: String,
    country: String,
    latitude: isize,
    longitude: isize,
    region: String,
    speed: usize,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Network {
    bluetooth: bool,
    carrier: String,
    cellular: bool,
    wifi: bool,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Os {
    name: String,
    version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Referrer {
    #[serde(rename = "type")]
    referrer_type: String,
    name: String,
    url: String,
    link: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeviceScreen {
    #[serde(rename = "type")]
    density: usize,
    height: usize,
    width: usize,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Traits {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<TraitAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<DateTime<Utc>>, // Date does not impl Serialize..

    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<TraitCompany>,

    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,

    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TraitCompany {
    name: Option<String>,
    id: Option<Value>, // TODO: make string or number enum to reduce variants
    industry: Option<String>,
    employee_count: Option<usize>,
    plan: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TraitAddress {
    city: Option<String>,
    country: Option<String>,
    postalCode: Option<String>,
    state: Option<String>,
    street: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    traits: Option<Traits>,

    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
enum IdentifyingID {
    Id {
        #[serde(rename = "userId")]
        value: String,
    },
    AnonymousId {
        #[serde(rename = "anonymousId")]
        value: String,
    },
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identify {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    id: IdentifyingID,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Track {
    #[serde(rename = "userId")]
    pub user_id: String,

    pub event: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Page {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Screen {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Alias {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "previousId")]
    pub previous_id: String,
}

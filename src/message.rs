use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

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
    pub context: Option<Context>,
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
    pub name: String,
    pub version: String,
    pub build: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Campaign {
    pub name: String,
    pub source: String,
    pub medium: String,
    pub term: String,
    pub content: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Device {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub name: String,

    #[serde(rename = "type")]
    pub device_type: String,

    pub version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Library {
    pub name: String,
    pub version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: isize,
    pub longitude: isize,
    pub region: String,
    pub speed: usize,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Network {
    pub bluetooth: bool,
    pub carrier: String,
    pub cellular: bool,
    pub wifi: bool,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Os {
    pub name: String,
    pub version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Webpage {
    pub hash: String,
    pub path: String,
    pub referrer: String,
    pub search: String,
    pub title: String,
    pub url: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Referrer {
    #[serde(rename = "type")]
    pub referrer_type: String,
    pub name: String,
    pub url: String,
    pub link: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeviceScreen {
    #[serde(rename = "type")]
    pub density: usize,
    pub height: usize,
    pub width: usize,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdentifyTraits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TraitAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<DateTime<Utc>>, // Date does not impl Serialize..

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<TraitCompany>,

    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TraitCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>, // TODO: make string or number enum to reduce variants

    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TraitAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Context {
    /// Whether a user is active
    ///
    /// This is usually used to flag an .identify() call to just update the traits but not “last seen.”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<Library>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Os>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Webpage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Referrer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<DeviceScreen>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<IdentifyTraits>,

    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum IdentifyingID {
    Id {
        #[serde(rename = "userId")]
        id: String,
    },
    AnonymousId {
        #[serde(rename = "anonymousId")]
        id: String,
    },
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identify {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub traits: Option<IdentifyTraits>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TrackProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Track {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    pub event: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TrackProperties>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Page {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    pub name: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PageProperties>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ScreenProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Screen {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    pub name: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScreenProperties>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GroupTraits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TraitAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub traits: Option<GroupTraits>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Alias {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "previousId")]
    pub previous_id: String,
}

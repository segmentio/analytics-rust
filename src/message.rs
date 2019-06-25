use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

/// Information about the current application, containing name, version and build.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct App {
    pub name: String,
    pub version: String,
    pub build: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

///
/// Information about the campaign that resulted in the API call.
///
/// This maps directly to the common UTM campaign parameters.
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

/// Information about the device, containing id, manufacturer, model, name, type and version.
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

/// Information about the library making the requests to the API, containing name and version.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Library {
    pub name: String,
    pub version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

/// Information about the user’s current location, containing city, country, latitude, longitude, region and speed.
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

/// Information about the current network connection, containing bluetooth, carrier, cellular and wifi.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Network {
    pub bluetooth: bool,
    pub carrier: String,
    pub cellular: bool,
    pub wifi: bool,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

/// Information about the operating system, containing name and version.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Os {
    pub name: String,
    pub version: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

/// Information about the current page in the browser, containing hash, path, referrer, search, title and url.
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

/// Information about the way the user was referred to the website or app, containing type, name, url and link.
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

/// Information about the device’s screen, containing density, height and width.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeviceScreen {
    #[serde(rename = "type")]
    pub density: usize,
    pub height: usize,
    pub width: usize,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

/// Contains reserved Traits to be included in any identify call.
///
/// You should only use reserved traits for their intended meaning.
///
/// Additional trait information should be added to the custom map field.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IdentifyTraits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TraitAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<usize>,

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

/// Context is a dictionary of extra information that provides useful context about a datapoint, for example the user’s ip address or locale. Context is a complete and explicit specification, so properties outside the spec will be ignored. You should only use Context fields for their intended meaning.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Context {
    /// Whether a user is active
    ///
    /// This is usually used to flag an .identify() call to just update the traits but not “last seen.”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Information about the current application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    /// Information about the campaign that resulted in the API call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,

    /// Information about the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    /// Current user’s IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Information about the library making the requests to the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<Library>,

    /// Locale string for the current user, for example `en-US`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// Information about the user’s current location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Information about the current network connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    /// Information about the operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Os>,

    /// Information about the current page in the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Webpage>,

    /// Information about the way the user was referred to the website or app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<Referrer>,

    /// Information about the device’s screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<DeviceScreen>,

    /// Timezones are sent as tzdata strings to add user timezone information which might be stripped from the timestamp.
    ///
    /// Ex: `America/New_York`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// Group / Account ID.
    ///
    /// This is useful in B2B use cases where you need to attribute your non-group calls to a company or account. It is relied on by several Customer Success and CRM tools.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// Traits of the current user.
    ///
    /// This is useful in cases where you need to track an event, but also associate information from a previous identify call. You should fill this object the same way you would fill traits in an identify call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<IdentifyTraits>,

    /// User agent of the device making the request.
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// Additional custom information that can be passed.
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

/// Identify lets you tie a user to their actions and record traits about them. It includes a unique User ID and any optional traits you know about them like their email, name, etc.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identify {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

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

impl Default for Identify {
    fn default() -> Self {
        Self {
            message_type: "identify".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            context: None,
            integrations: None,
            timestamp: None,
            traits: None,
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Track {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

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

impl Default for Track {
    fn default() -> Self {
        Self {
            message_type: "track".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            context: None,
            event: "".to_owned(),
            integrations: None,
            properties: None,
            timestamp: None,
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Page {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

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

impl Default for Page {
    fn default() -> Self {
        Self {
            message_type: "page".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            context: None,
            name: "".to_owned(),
            integrations: None,
            properties: None,
            timestamp: None,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ScreenProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Screen {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

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

impl Default for Screen {
    fn default() -> Self {
        Self {
            message_type: "screen".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            context: None,
            name: "".to_string(),
            integrations: None,
            properties: None,
            timestamp: None,
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

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

impl Default for Group {
    fn default() -> Self {
        Self {
            message_type: "group".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            context: None,
            group_id: "".to_owned(),
            integrations: None,
            timestamp: None,
            traits: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Alias {
    #[serde(rename = "type")]
    pub(crate) message_type: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub id: Option<IdentifyingID>,

    #[serde(rename = "previousId")]
    pub previous_id: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<BTreeMap<String, bool>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

impl Default for Alias {
    fn default() -> Self {
        Self {
            message_type: "alias".to_owned(),
            message_id: Uuid::new_v4().to_string(),
            id: None,
            previous_id: "".to_owned(),
            context: None,
            integrations: None,
            timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let alias = Alias {
            message_id: "myid".to_owned(),
            ..Default::default()
        };
        assert_eq!("myid".to_owned(), alias.message_id);
        assert_eq!("alias".to_owned(), alias.message_type);
    }
}

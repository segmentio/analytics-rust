use chrono::{DateTime, Utc};
use failure::format_err;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

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

macro_rules! str_setter {
    ($id:ident) => {
        pub fn $id<S>(mut self, $id: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let value = $id.into().trim().to_owned();
        if value.len() == 0 {
            return Err(format_err!("{} must contains a value", stringify!($id)));
        }
        (self.0).$id = value;
        Ok(self)
    }
    };
}

macro_rules! object_setter {
    ($id:ident,$t:ty) => {
        pub fn $id(mut self, $id: $t) -> Result<Self, Error>
        {
            (self.0).$id = Some($id);
            Ok(self)
        }
    };
}

macro_rules! str_option_setter {
    ($id:ident) => {
        pub fn $id<S>(mut self, $id: S) -> Result<Self, Error>
        where
            S: Into<String>,
        {
            let value = $id.into().trim().to_owned();
            if value.len() == 0 {
                return Err(format_err!("{} must contains a value", stringify!($id)));
            }
            (self.0).$id = Some(value);
            Ok(self)
        }
    };
}

macro_rules! common_setters {
    () => {
        str_setter!(message_id);
        str_option_setter!(anonymous_id);
        str_option_setter!(user_id);
        object_setter!(context, Context);
        object_setter!(integrations, Integrations);
        object_setter!(timestamp, DateTime<Utc>);
    };
}

/// `Message` represents the valid message types for which the Segment API supports.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "identify")]
    Identify(Identify),

    #[serde(rename = "track")]
    Track(Track),

    #[serde(rename = "page")]
    Page(Page),

    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "screen")]
    Screen(Screen),

    #[serde(rename = "alias")]
    Alias(Alias),

    #[serde(rename = "batch")]
    Batch(Batch),
}

/// `Batch` represents the batch payload for the APIs `/v1/batch` endpoint.
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

/// `BatchMessage` represents the message types that are supported to be send via the API's `/v1/batch` endpoint.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BatchMessage {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
}

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

/// `IdentifyTraits` representsthe traits that can be included in any identify call.
///
/// **You should only use reserved traits for their intended meaning.**
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

/// `TraitCompany` represents the the trait information that are available for a company.
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

/// `TraitAddress` represents the the trait information that are available for an address.
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

/// `Context` is a set of extra information that provides useful context about a datapoint, for example the user’s ip address or locale.
/// Context is a complete and explicit specification, so properties outside the spec will be ignored.
///
/// **You should only use Context fields for their intended meaning.**
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

pub struct IdentifyBuilder(Identify);

impl IdentifyBuilder {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(Identify {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            context: None,
            integrations: Some(Integrations::default()),
            timestamp: None,
            traits: None,
        }))
    }

    common_setters!();
    object_setter!(traits, IdentifyTraits);

    pub fn build(self) -> Result<Identify, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

/// Identify lets you tie a user to their actions and record traits about them. It includes a unique User ID and any optional traits you know about them like their email, name, etc.
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identify {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// A set of traits of the user, like email or name.
    ///
    /// See the [Traits field docs](https://segment.com/docs/spec/identify#traits) for a list of reserved trait names
    #[serde(skip_serializing_if = "Option::is_none")]
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

pub struct TrackBuilder(Track);

impl TrackBuilder {
    pub fn new<S>(event: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let evt = event.into().trim().to_owned();
        if evt.len() == 0 {
            return Err(format_err!("event must contain a value"));
        }
        Ok(Self(Track {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            context: None,
            event: evt,
            integrations: Some(Integrations::default()),
            properties: None,
            timestamp: None,
        }))
    }

    common_setters!();
    object_setter!(properties, TrackProperties);

    pub fn build(self) -> Result<Track, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Track {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Name of the action that a user has performed.
    ///
    /// See the [Event field docs](https://segment.com/docs/spec/track#event) for more detail
    pub event: String,

    /// A set of properties of the screen, like name
    ///
    /// See the [Properties field docs](https://segment.com/docs/spec/screen#properties) for a list of reserved property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TrackProperties>,
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

pub struct PageBuilder(Page);

impl PageBuilder {
    pub fn new<S>(name: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let n = name.into().trim().to_owned();
        if n.len() == 0 {
            return Err(format_err!("name must contain a value"));
        }
        Ok(Self(Page {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            context: None,
            name: n,
            integrations: Some(Integrations::default()),
            properties: None,
            timestamp: None,
        }))
    }

    common_setters!();
    object_setter!(properties, PageProperties);

    pub fn build(self) -> Result<Page, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Page {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Name of the page
    ///
    /// For example, most sites have a `Signup` page that can be useful to tag, so you can see users as they move through your funnel.
    pub name: String,

    /// A set of properties of the screen, like name
    ///
    /// See the [Properties field docs](https://segment.com/docs/spec/screen#properties) for a list of reserved property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PageProperties>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ScreenProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Map<String, Value>>,
}

pub struct ScreenBuilder(Screen);

impl ScreenBuilder {
    pub fn new<S>(name: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let n = name.into().trim().to_owned();
        if n.len() == 0 {
            return Err(format_err!("name must contain a value"));
        }
        Ok(Self(Screen {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            context: None,
            name: n,
            integrations: Some(Integrations::default()),
            properties: None,
            timestamp: None,
        }))
    }

    common_setters!();
    object_setter!(properties, ScreenProperties);

    pub fn build(self) -> Result<Screen, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Screen {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Name of the screen
    ///
    /// See the [Name field docs](https://segment.com/docs/spec/screen#name) for more detail
    pub name: String,

    /// A set of properties of the screen, like name
    ///
    /// See the [Properties field docs](https://segment.com/docs/spec/screen#properties) for a list of reserved property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScreenProperties>,
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

pub struct GroupBuilder(Group);

impl GroupBuilder {
    pub fn new<S>(group_id: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let g_id = group_id.into().trim().to_owned();
        if g_id.len() == 0 {
            return Err(format_err!("group_id must contain a value"));
        }

        Ok(Self(Group {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            context: None,
            group_id: g_id,
            integrations: Some(Integrations::default()),
            timestamp: None,
            traits: None,
        }))
    }

    common_setters!();
    object_setter!(traits, GroupTraits);

    pub fn build(self) -> Result<Group, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// A unique identifier for the group in your database.
    ///
    /// See the [Group ID field docs](https://segment.com/docs/spec/group#group-id) for more detail
    #[serde(rename = "groupId")]
    pub group_id: String,

    /// A set of traits of the group, like email or name
    ///
    /// See the [Traits field docs](https://segment.com/docs/spec/group#traits) for a list of reserved trait names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<GroupTraits>,
}

pub struct AliasBuilder(Alias);

impl AliasBuilder {
    pub fn new<S>(previous_id: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let prev_id = previous_id.into().trim().to_owned();
        if prev_id.len() == 0 {
            return Err(format_err!("previous_id must contain a value"));
        }

        Ok(Self(Alias {
            message_id: Uuid::new_v4().to_string(),
            anonymous_id: None,
            user_id: None,
            previous_id: prev_id,
            context: None,
            integrations: Some(Integrations::default()),
            timestamp: None,
        }))
    }

    common_setters!();

    pub fn build(self) -> Result<Alias, Error> {
        if self.0.anonymous_id.is_none() && self.0.user_id.is_none() {
            return Err(format_err!("an anonymous_id or user_id must be set"));
        }
        Ok(self.0)
    }
}

/// Alias is how you associate one identity with another. This is an advanced method, but it is required to manage user identities successfully in some destinations.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Alias {
    /// a unique identifier for each message that lets you find an individual message across the API.
    #[serde(rename = "messageId")]
    pub message_id: String,

    /// A pseudo-unique substitute for a User ID, for cases when you don’t have an absolutely unique identifier. A userId or an anonymousId is required.
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    /// Unique identifier for the user in your database
    ///
    /// A userId or an anonymousId is required
    ///
    /// See the [Identities docs](https://segment.com/docs/spec/identify/#identities) for more detail
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Context is a set of extra information that provides useful context about a datapoint,
    /// for example the user’s ip address or locale.
    ///
    /// Context is a complete and explicit specification, so properties outside the spec will be ignored.
    /// You should only use Context fields for their intended meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    /// A set of destination names that the message should be sent to. 'All' is a special key that applies when no key for a specific destination is found.
    ///
    /// Integrations defaults to the following:
    ///
    /// ```json
    ///{
    ///  All: true,
    ///  Salesforce: false,
    ///}
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Integrations>,

    /// Used by Segment to send to downstream destinations and for historical replays.
    ///
    /// **Note:** Recommended timestamp for analysis when chronology does matter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Previous unique identifier for the user
    ///
    /// See the [Previous ID field docs](https://segment.com/docs/spec/alias#previous-id) for more detail
    #[serde(rename = "previousId")]
    pub previous_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Integrations(BTreeMap<String, bool>);

impl Default for Integrations {
    fn default() -> Self {
        let mut integrations = BTreeMap::new();
        integrations.insert("All".to_owned(), true);
        integrations.insert("Salesforce".to_owned(), false);
        Integrations(integrations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias() -> Result<(), Error> {
        let alias = AliasBuilder::new("prev_id")?
            .message_id("myid")?
            .anonymous_id("anon")?
            .build()?;

        assert_eq!("myid".to_owned(), alias.message_id);
        //
        let msg = Message::Alias(alias);
        let res = serde_json::to_string(&msg).unwrap();
        assert_eq!(
            r#"{"type":"alias","messageId":"myid","anonymousId":"anon","integrations":{"All":true,"Salesforce":false},"previousId":"prev_id"}"#
                .to_owned(),
            res
        );
        Ok(())
    }

    #[test]
    fn test_bad_alias() -> Result<(), Error> {
        let alias = AliasBuilder::new("");
        assert_eq!(true, alias.is_err());
        Ok(())
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Identify {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The traits to assign to the user.
    pub traits: Value,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Track {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The name of the event being tracked.
    pub event: String,

    /// The properties associated with the event.
    pub properties: Value,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Page {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The name of the page being tracked.
    pub name: String,

    /// The properties associated with the event.
    pub properties: Value,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Screen {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The name of the screen being tracked.
    pub name: String,

    /// The properties associated with the event.
    pub properties: Value,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The group the user is being associated with.
    #[serde(rename = "groupId")]
    pub group_id: String,

    /// The traits to assign to the group.
    pub traits: Value,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Alias {
    /// The user associated with this message.
    #[serde(flatten)]
    pub user: User,

    /// The user's previous ID.
    #[serde(rename = "previousId")]
    pub previous_id: String,

    /// The timestamp associated with this message.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub timestamp: Option<OffsetDateTime>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Batch {
    /// The batch of messages to send.
    pub batch: Vec<BatchMessage>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// Extra fields to put at the top level of this message.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchMessage {
    #[serde(rename = "identify")]
    Identify(Identify),
    #[serde(rename = "track")]
    Track(Track),
    #[serde(rename = "page")]
    Page(Page),
    #[serde(rename = "screen")]
    Screen(Screen),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "alias")]
    Alias(Alias),
}

impl BatchMessage {
    pub(crate) fn timestamp_mut(&mut self) -> &mut Option<OffsetDateTime> {
        match self {
            Self::Identify(identify) => &mut identify.timestamp,
            Self::Track(track) => &mut track.timestamp,
            Self::Page(page) => &mut page.timestamp,
            Self::Screen(screen) => &mut screen.timestamp,
            Self::Group(group) => &mut group.timestamp,
            Self::Alias(alias) => &mut alias.timestamp,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum User {
    /// The user is identified only by a user ID.
    UserId {
        #[serde(rename = "userId")]
        user_id: String,
    },

    /// The user is identified only by an anonymous ID.
    AnonymousId {
        #[serde(rename = "anonymousId")]
        anonymous_id: String,
    },

    /// The user is identified by both a user ID and an anonymous ID.
    Both {
        #[serde(rename = "userId")]
        user_id: String,

        #[serde(rename = "anonymousId")]
        anonymous_id: String,
    },
}

impl Display for User {
    /// Display a `UserId`. If he has both an `anonymous_id` and a `user_id` we display the `user_id`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            User::UserId { user_id } => write!(f, "{}", user_id),
            User::AnonymousId { anonymous_id } => write!(f, "{}", anonymous_id),
            User::Both { user_id, .. } => write!(f, "{}", user_id),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::AnonymousId {
            anonymous_id: "".to_owned(),
        }
    }
}

macro_rules! into {
    (from $from:ident into $for:ident) => {
        impl From<$from> for $for {
            fn from(message: $from) -> Self {
                Self::$from(message)
            }
        }
    };
    ($(from $from:ident into $for:ident),+ $(,)?) => {
        $(
            into!{from $from into $for}
        )+
    };
}

into! {
    from Identify into Message,
    from Track into Message,
    from Page into Message,
    from Screen into Message,
    from Group into Message,
    from Alias into Message,
    from Batch into Message,

    from Identify into BatchMessage,
    from Track into BatchMessage,
    from Page into BatchMessage,
    from Screen into BatchMessage,
    from Group into BatchMessage,
    from Alias into BatchMessage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&Message::Identify(Identify {
                user: User::UserId {
                    user_id: "foo".to_owned()
                },
                traits: json!({
                    "foo": "bar",
                    "baz": "quux",
                }),
                extra: [("messageId".to_owned(), json!("123"))]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","traits":{"baz":"quux","foo":"bar"},"messageId":"123"}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Track(Track {
                user: User::AnonymousId {
                    anonymous_id: "foo".to_owned()
                },
                event: "Foo".to_owned(),
                properties: json!({
                    "foo": "bar",
                    "baz": "quux",
                }),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"anonymousId":"foo","event":"Foo","properties":{"baz":"quux","foo":"bar"}}"#
                .to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Page(Page {
                user: User::Both {
                    user_id: "foo".to_owned(),
                    anonymous_id: "bar".to_owned()
                },
                name: "Foo".to_owned(),
                properties: json!({
                    "foo": "bar",
                    "baz": "quux",
                }),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","anonymousId":"bar","name":"Foo","properties":{"baz":"quux","foo":"bar"}}"#
                .to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Screen(Screen {
                user: User::Both {
                    user_id: "foo".to_owned(),
                    anonymous_id: "bar".to_owned()
                },
                name: "Foo".to_owned(),
                properties: json!({
                    "foo": "bar",
                    "baz": "quux",
                }),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","anonymousId":"bar","name":"Foo","properties":{"baz":"quux","foo":"bar"}}"#
                .to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Group(Group {
                user: User::UserId {
                    user_id: "foo".to_owned()
                },
                group_id: "bar".to_owned(),
                traits: json!({
                    "foo": "bar",
                    "baz": "quux",
                }),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","groupId":"bar","traits":{"baz":"quux","foo":"bar"}}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Alias(Alias {
                user: User::UserId {
                    user_id: "foo".to_owned()
                },
                previous_id: "bar".to_owned(),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","previousId":"bar"}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Batch(Batch {
                batch: vec![
                    BatchMessage::Track(Track {
                        user: User::UserId {
                            user_id: "foo".to_owned()
                        },
                        event: "Foo".to_owned(),
                        properties: json!({}),
                        ..Default::default()
                    }),
                    BatchMessage::Track(Track {
                        user: User::UserId {
                            user_id: "bar".to_owned()
                        },
                        event: "Bar".to_owned(),
                        properties: json!({}),
                        ..Default::default()
                    }),
                    BatchMessage::Track(Track {
                        user: User::UserId {
                            user_id: "baz".to_owned()
                        },
                        event: "Baz".to_owned(),
                        properties: json!({}),
                        ..Default::default()
                    })
                ],
                context: Some(json!({
                    "foo": "bar",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"batch":[{"type":"track","userId":"foo","event":"Foo","properties":{}},{"type":"track","userId":"bar","event":"Bar","properties":{}},{"type":"track","userId":"baz","event":"Baz","properties":{}}],"context":{"foo":"bar"}}"#
                .to_owned(),
        );
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Identify {
    #[serde(flatten)]
    pub user: User,
    pub traits: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Track {
    #[serde(flatten)]
    pub user: User,
    pub event: String,
    pub properties: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Page {
    #[serde(flatten)]
    pub user: User,
    pub name: String,
    pub properties: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Screen {
    #[serde(flatten)]
    pub user: User,
    pub name: String,
    pub properties: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    #[serde(flatten)]
    pub user: User,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub traits: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Alias {
    #[serde(flatten)]
    pub user: User,
    #[serde(rename = "previousId")]
    pub previous_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Batch {
    pub batch: Vec<BatchMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum User {
    UserId {
        #[serde(rename = "userId")]
        user_id: String,
    },
    AnonymousId {
        #[serde(rename = "anonymousId")]
        anonymous_id: String,
    },
    Both {
        #[serde(rename = "userId")]
        user_id: String,

        #[serde(rename = "anonymousId")]
        anonymous_id: String,
    },
}

impl Default for User {
    fn default() -> Self {
        User::AnonymousId {
            anonymous_id: "".to_owned(),
        }
    }
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
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","traits":{"baz":"quux","foo":"bar"}}"#.to_owned(),
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

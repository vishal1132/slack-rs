use super::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub ok: bool,
    pub messages: Vec<Message>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "pin_count")]
    pub pin_count: i64,
    #[serde(rename = "channel_actions_ts")]
    pub channel_actions_ts: Value,
    #[serde(rename = "channel_actions_count")]
    pub channel_actions_count: i64,
    #[serde(rename = "response_metadata")]
    #[serde(default)]
    pub response_metadata: ResponseMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(rename = "next_cursor")]
    #[serde(default)]
    pub next_cursor: String,
}

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
    pub response_metadata: ResponseMetadata,
}

const DEFAULT_REPLY_COUNT: i64 = 0;
const DEFAULT_REPLY_USERS_COUNT: i64 = 0;

fn default_reply_users_count() -> i64 {
    DEFAULT_REPLY_USERS_COUNT
}

fn default_reply_count() -> i64 {
    DEFAULT_REPLY_COUNT
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub text: String,
    // #[serde(default)]
    // pub files: Vec<File>,
    pub upload: Option<bool>,
    pub user: String,
    #[serde(rename = "display_as_bot")]
    pub display_as_bot: Option<bool>,
    // #[serde(default)]
    // pub blocks: Vec<Block>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ts: String,
    #[serde(default)]
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(default)]
    #[serde(rename = "thread_ts")]
    pub thread_ts: String,
    #[serde(default = "default_reply_count")]
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(default = "default_reply_users_count")]
    #[serde(rename = "reply_users_count")]
    pub reply_users_count: i64,
    #[serde(default)]
    #[serde(rename = "latest_reply")]
    pub latest_reply: String,
    #[serde(default)]
    #[serde(rename = "reply_users")]
    pub reply_users: Vec<String>,
    #[serde(default)]
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    #[serde(default)]
    pub subscribed: bool,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    pub team: Option<String>,
    pub edited: Option<Edited>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub created: i64,
    pub timestamp: i64,
    pub name: String,
    pub title: String,
    pub mimetype: String,
    pub filetype: String,
    #[serde(rename = "pretty_type")]
    pub pretty_type: String,
    pub user: String,
    #[serde(rename = "user_team")]
    pub user_team: String,
    pub editable: bool,
    pub size: i64,
    pub mode: String,
    #[serde(rename = "is_external")]
    pub is_external: bool,
    #[serde(rename = "external_type")]
    pub external_type: String,
    #[serde(rename = "is_public")]
    pub is_public: bool,
    #[serde(rename = "public_url_shared")]
    pub public_url_shared: bool,
    #[serde(rename = "display_as_bot")]
    pub display_as_bot: bool,
    pub username: String,
    #[serde(rename = "url_private")]
    pub url_private: String,
    #[serde(rename = "url_private_download")]
    pub url_private_download: String,
    #[serde(rename = "media_display_type")]
    pub media_display_type: String,
    #[serde(rename = "thumb_64")]
    pub thumb_64: String,
    #[serde(rename = "thumb_80")]
    pub thumb_80: String,
    #[serde(rename = "thumb_360")]
    pub thumb_360: String,
    #[serde(rename = "thumb_360_w")]
    pub thumb_360_w: i64,
    #[serde(rename = "thumb_360_h")]
    pub thumb_360_h: i64,
    #[serde(rename = "thumb_480")]
    pub thumb_480: String,
    #[serde(rename = "thumb_480_w")]
    pub thumb_480_w: i64,
    #[serde(rename = "thumb_480_h")]
    pub thumb_480_h: i64,
    #[serde(rename = "thumb_160")]
    pub thumb_160: String,
    #[serde(rename = "thumb_720")]
    pub thumb_720: String,
    #[serde(rename = "thumb_720_w")]
    pub thumb_720_w: i64,
    #[serde(rename = "thumb_720_h")]
    pub thumb_720_h: i64,
    #[serde(rename = "thumb_800")]
    pub thumb_800: String,
    #[serde(rename = "thumb_800_w")]
    pub thumb_800_w: i64,
    #[serde(rename = "thumb_800_h")]
    pub thumb_800_h: i64,
    #[serde(rename = "thumb_960")]
    pub thumb_960: String,
    #[serde(rename = "thumb_960_w")]
    pub thumb_960_w: i64,
    #[serde(rename = "thumb_960_h")]
    pub thumb_960_h: i64,
    #[serde(rename = "thumb_1024")]
    pub thumb_1024: String,
    #[serde(rename = "thumb_1024_w")]
    pub thumb_1024_w: i64,
    #[serde(rename = "thumb_1024_h")]
    pub thumb_1024_h: i64,
    #[serde(rename = "original_w")]
    pub original_w: i64,
    #[serde(rename = "original_h")]
    pub original_h: i64,
    #[serde(rename = "thumb_tiny")]
    pub thumb_tiny: String,
    pub permalink: String,
    #[serde(rename = "permalink_public")]
    pub permalink_public: String,
    #[serde(rename = "is_starred")]
    pub is_starred: bool,
    #[serde(rename = "has_rich_preview")]
    pub has_rich_preview: bool,
    #[serde(rename = "file_access")]
    pub file_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "block_id")]
    pub block_id: String,
    pub elements: Vec<Element>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "type")]
    pub type_field: String,
    pub elements: Vec<Element2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub style: Option<Style>,
    pub name: Option<String>,
    pub unicode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub italic: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edited {
    pub user: String,
    pub ts: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub ok: bool,
    pub messages: Vec<Message>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub user: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ts: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    pub text: String,
    pub team: Option<String>,
    #[serde(rename = "thread_ts")]
    pub thread_ts: String,
    #[serde(rename = "reply_count")]
    pub reply_count: Option<i64>,
    #[serde(rename = "reply_users_count")]
    pub reply_users_count: Option<i64>,
    #[serde(rename = "latest_reply")]
    pub latest_reply: Option<String>,
    #[serde(rename = "reply_users")]
    pub reply_users: Option<Vec<String>>,
    #[serde(rename = "is_locked")]
    pub is_locked: Option<bool>,
    pub subscribed: Option<bool>,
    pub blocks: Vec<Block>,
    pub subtype: Option<String>,
    pub root: Option<Root2>,
    pub edited: Option<Edited>,
    #[serde(rename = "parent_user_id")]
    pub parent_user_id: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
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
    pub url: Option<String>,
    pub style: Option<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub code: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub user: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ts: String,
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    pub text: String,
    pub team: String,
    #[serde(rename = "thread_ts")]
    pub thread_ts: String,
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(rename = "reply_users_count")]
    pub reply_users_count: i64,
    #[serde(rename = "latest_reply")]
    pub latest_reply: String,
    #[serde(rename = "reply_users")]
    pub reply_users: Vec<String>,
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    pub subscribed: bool,
    pub blocks: Vec<Block2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "block_id")]
    pub block_id: String,
    pub elements: Vec<Element3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub elements: Vec<Element4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edited {
    pub user: String,
    pub ts: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "from_url")]
    pub from_url: String,
    pub ts: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "channel_team")]
    pub channel_team: String,
    #[serde(rename = "is_msg_unfurl")]
    pub is_msg_unfurl: bool,
    #[serde(rename = "is_reply_unfurl")]
    pub is_reply_unfurl: bool,
    #[serde(rename = "message_blocks")]
    pub message_blocks: Vec<MessageBlock>,
    pub id: i64,
    #[serde(rename = "original_url")]
    pub original_url: String,
    pub fallback: String,
    pub text: String,
    #[serde(rename = "author_name")]
    pub author_name: String,
    #[serde(rename = "author_link")]
    pub author_link: String,
    #[serde(rename = "author_icon")]
    pub author_icon: String,
    #[serde(rename = "author_subname")]
    pub author_subname: String,
    #[serde(rename = "mrkdwn_in")]
    pub mrkdwn_in: Vec<String>,
    pub footer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageBlock {
    pub team: String,
    pub channel: String,
    pub ts: String,
    pub message: Message2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message2 {
    pub blocks: Vec<Block3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "block_id")]
    pub block_id: String,
    pub elements: Vec<Element5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element5 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub elements: Vec<Element6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element6 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<String>,
    pub name: Option<String>,
    pub unicode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: i64,
}

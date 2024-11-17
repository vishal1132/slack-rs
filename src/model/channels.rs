use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub channels: Vec<Channel>,
    #[serde(default, rename = "response_metadata")]
    pub response_metadata: ResponseMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "is_channel")]
    pub is_channel: bool,
    #[serde(default, rename = "is_group")]
    pub is_group: bool,
    #[serde(default, rename = "is_im")]
    pub is_im: bool,
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub creator: String,
    #[serde(default, rename = "is_archived")]
    pub is_archived: bool,
    #[serde(default, rename = "is_general")]
    pub is_general: bool,
    #[serde(default)]
    pub unlinked: i64,
    #[serde(default, rename = "name_normalized")]
    pub name_normalized: String,
    #[serde(default, rename = "is_shared")]
    pub is_shared: bool,
    #[serde(default, rename = "is_ext_shared")]
    pub is_ext_shared: bool,
    #[serde(default, rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(default, rename = "pending_shared")]
    pub pending_shared: Vec<Value>,
    #[serde(default, rename = "is_pending_ext_shared")]
    pub is_pending_ext_shared: bool,
    #[serde(default, rename = "is_member")]
    pub is_member: bool,
    #[serde(default, rename = "is_private")]
    pub is_private: bool,
    #[serde(default, rename = "is_mpim")]
    pub is_mpim: bool,
    #[serde(default)]
    pub updated: i64,
    #[serde(default)]
    pub topic: Topic,
    #[serde(default)]
    pub purpose: Purpose,
    #[serde(default, rename = "previous_names")]
    pub previous_names: Vec<Value>,
    #[serde(default, rename = "num_members")]
    pub num_members: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub creator: String,
    #[serde(default, rename = "last_set")]
    pub last_set: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Purpose {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub creator: String,
    #[serde(default, rename = "last_set")]
    pub last_set: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(default, rename = "next_cursor")]
    pub next_cursor: String,
}

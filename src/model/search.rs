use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default)]
    pub messages: Messages,
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub query: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Messages {
    #[serde(default)]
    pub matches: Vec<Match>,
    #[serde(default)]
    pub pagination: Pagination,
    #[serde(default)]
    pub paging: Paging,
    #[serde(default)]
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    #[serde(default)]
    pub channel: Channel,
    #[serde(default)]
    pub iid: String,
    #[serde(default)]
    pub permalink: String,
    #[serde(default)]
    pub team: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub ts: String,
    #[serde(default, rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(default)]
    pub id: String,
    #[serde(default, rename = "is_ext_shared")]
    pub is_ext_shared: bool,
    #[serde(default, rename = "is_mpim")]
    pub is_mpim: bool,
    #[serde(default, rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(default, rename = "is_pending_ext_shared")]
    pub is_pending_ext_shared: bool,
    #[serde(default, rename = "is_private")]
    pub is_private: bool,
    #[serde(default, rename = "is_shared")]
    pub is_shared: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "pending_shared")]
    pub pending_shared: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default)]
    pub first: i64,
    #[serde(default)]
    pub last: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default, rename = "page_count")]
    pub page_count: i64,
    #[serde(default, rename = "per_page")]
    pub per_page: i64,
    #[serde(default, rename = "total_count")]
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub pages: i64,
    #[serde(default)]
    pub total: i64,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub user: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub ts: String,
    #[serde(default)]
    #[serde(rename = "client_msg_id")]
    pub client_msg_id: String,
    #[serde(default)]
    pub text: String,
    pub team: Option<String>,
}

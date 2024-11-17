use super::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub ok: bool,
    pub messages: Vec<Message>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

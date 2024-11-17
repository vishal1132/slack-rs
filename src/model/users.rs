use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub members: Vec<Member>,
    #[serde(rename = "cache_ts", default)]
    pub cache_ts: i64,
    #[serde(rename = "response_metadata", default)]
    pub response_metadata: ResponseMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "team_id", default)]
    pub team_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub color: String,
    #[serde(rename = "real_name", default)]
    pub real_name: String,
    #[serde(default)]
    pub tz: String,
    #[serde(rename = "tz_label", default)]
    pub tz_label: String,
    #[serde(rename = "tz_offset", default)]
    pub tz_offset: i64,
    #[serde(default)]
    pub profile: Profile,
    #[serde(rename = "is_admin", default)]
    pub is_admin: bool,
    #[serde(rename = "is_owner", default)]
    pub is_owner: bool,
    #[serde(rename = "is_primary_owner", default)]
    pub is_primary_owner: bool,
    #[serde(rename = "is_restricted", default)]
    pub is_restricted: bool,
    #[serde(rename = "is_ultra_restricted", default)]
    pub is_ultra_restricted: bool,
    #[serde(rename = "is_bot", default)]
    pub is_bot: bool,
    #[serde(default)]
    pub updated: i64,
    #[serde(rename = "is_app_user", default)]
    pub is_app_user: Option<bool>,
    #[serde(rename = "has_2fa", default)]
    pub has_2fa: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "avatar_hash", default)]
    pub avatar_hash: String,
    #[serde(rename = "status_text", default)]
    pub status_text: Option<String>,
    #[serde(rename = "status_emoji", default)]
    pub status_emoji: Option<String>,
    #[serde(rename = "real_name", default)]
    pub real_name: String,
    #[serde(rename = "display_name", default)]
    pub display_name: String,
    #[serde(rename = "real_name_normalized", default)]
    pub real_name_normalized: String,
    #[serde(rename = "display_name_normalized", default)]
    pub display_name_normalized: String,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "image_24", default)]
    pub image_24: String,
    #[serde(rename = "image_32", default)]
    pub image_32: String,
    #[serde(rename = "image_48", default)]
    pub image_48: String,
    #[serde(rename = "image_72", default)]
    pub image_72: String,
    #[serde(rename = "image_192", default)]
    pub image_192: String,
    #[serde(rename = "image_512", default)]
    pub image_512: String,
    #[serde(default)]
    pub team: Option<String>,
    #[serde(rename = "image_1024", default)]
    pub image_1024: Option<String>,
    #[serde(rename = "image_original", default)]
    pub image_original: Option<String>,
    #[serde(rename = "first_name", default)]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub skype: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

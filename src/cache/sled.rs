use crate::cache::cache::Cache;
use crate::model::{Channel, User};
use std::error::Error;

const DB_PATH: &str = "/Users/vishalsharma/.slack/sled.db";

/// sled db with schema
/// users table-> user_id, user_name, team_name
/// channels table-> channel_id, channel_name, team_name
impl Cache for InMemoryCache {
    fn get_user(&self, team: &str, id: &str) -> Option<User> {
        None
    }

    fn get_channel(&self, team: &str, id: &str) -> Option<Channel> {
        None
    }

    fn get_channel_id(&self, team: &str, name: &str) -> Option<String> {
        None
    }

    fn get_user_id(&self, team: &str, name: &str) -> Option<String> {
        None
    }
}

pub struct InMemoryCache {}

impl InMemoryCache {
    pub fn new() -> Result<Box<dyn Cache>, Box<dyn Error>> {
        let db = sled::open(DB_PATH)?;
        Ok(Box::new(InMemoryCache {}))
    }
}

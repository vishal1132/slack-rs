use crate::model::domain::*;
use std::error::Error;
pub trait Cache {
    fn get_user(&self, team: &str, id: &str) -> Option<User>;
    fn get_channel(&self, team: &str, id: &str) -> Option<Channel>;
    // fn get_channel_id(&self, team: &str, name: &str) -> Option<String>;
    // fn get_user_id(&self, team: &str, name: &str) -> Option<String>;
    fn sync_channels(&self, team: &str, channels: Vec<Channel>) -> Result<(), Box<dyn Error>>;
    fn sync_users(&self, team: &str, users: Vec<User>) -> Result<(), Box<dyn Error>>;
}

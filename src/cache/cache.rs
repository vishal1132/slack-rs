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

// pub trait CacheClone {
//     fn clone_box(&self) -> Box<dyn Cache>;
// }

// impl<T> CacheClone for T
// where
//     T: 'static + Cache + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Cache> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn Cache> {
//     fn clone(&self) -> Box<dyn Cache> {
//         self.clone_box()
//     }
// }
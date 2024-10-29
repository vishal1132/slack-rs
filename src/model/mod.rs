pub mod conversations;
pub mod replies;

pub struct User {
    pub id: String,
    pub name: String,
}

pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
}

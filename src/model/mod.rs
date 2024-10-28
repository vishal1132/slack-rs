pub mod replies;
pub mod conversations;

pub struct User {
    pub id: String,
    pub name: String,
}

pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
}

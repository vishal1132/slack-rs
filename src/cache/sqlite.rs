use sqlite::State;

use crate::cache::cache::Cache;
use crate::model::domain::*;
use std::error::Error;
use std::fs;
use std::path::Path;

const DB_PATH: &str = "/Users/vishalsharma/.slack/info.db";

/// sqlite
/// users table-> user_id, user_name, team_name
/// channels table-> channel_id, channel_name, team_name
impl Cache for InMemoryCache {
    fn get_channel(&self, team: &str, id: &str) -> Option<Channel> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM channels WHERE id = ? and team = ?")
            .unwrap();
        statement.bind((1, id)).unwrap();
        statement.bind((2, team)).unwrap();
        if let State::Row = statement.next().unwrap() {
            let id = statement.read::<String, usize>(0).unwrap();
            let name = statement.read(1).unwrap();
            return Some(Channel {
                id,
                name,
                is_channel: true,
            });
        }
        None
    }

    fn get_user(&self, team: &str, id: &str) -> Option<User> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM users WHERE id = ? and team = ?")
            .unwrap();
        statement.bind((1, id)).unwrap();
        statement.bind((2, team)).unwrap();
        if let State::Row = statement.next().unwrap() {
            let id = statement.read::<String, usize>(0).unwrap();
            let name = statement.read(1).unwrap();
            return Some(User { id, name });
        }
        None
    }

    // fn get_channel_id(&self, team: &str, name: &str) -> Option<String> {
    //     None
    // }

    // fn get_user_id(&self, team: &str, name: &str) -> Option<String> {
    //     None
    // }

    fn sync_users(&self, team: &str, users: Vec<User>) -> Result<(), Box<dyn Error>> {
        self.connection.execute("BEGIN TRANSACTION")?;
        for user in users {
            let query = format!(
                "INSERT OR REPLACE INTO users (id, name, team) VALUES ('{}', '{}', '{}')",
                user.id, user.name, team
            );
            self.connection.execute(&query)?;
        }
        self.connection.execute("COMMIT")?;
        Ok(())
    }

    fn sync_channels(&self, team: &str, channels: Vec<Channel>) -> Result<(), Box<dyn Error>> {
        self.connection.execute("BEGIN TRANSACTION")?;
        for channel in channels {
            let query = format!(
                "INSERT OR REPLACE INTO channels (id, name, team) VALUES ('{}', '{}', '{}')",
                channel.id, channel.name, team
            );
            self.connection.execute(&query)?;
        }
        self.connection.execute("COMMIT")?;
        Ok(())
    }
}

pub struct InMemoryCache {
    connection: sqlite::Connection,
}

impl InMemoryCache {
    pub fn new() -> Result<Box<dyn Cache>, Box<dyn Error>> {
        if let Some(parent_dir) = Path::new(DB_PATH).parent() {
            fs::create_dir_all(parent_dir).unwrap();
        }
        if !Path::new(DB_PATH).exists() {
            std::fs::File::create(DB_PATH).unwrap();
        }
        let connection = sqlite::open(DB_PATH)?;
        let db = Box::new(InMemoryCache { connection });
        db.migrate()?;
        Ok(db)
    }

    pub fn migrate(&self) -> Result<(), Box<dyn Error>> {
        let query = "
        CREATE TABLE IF NOT EXISTS channels (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            team TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            team TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS audit (
            id INTEGER PRIMARY KEY,
            action TEXT NOT NULL,
            table_name TEXT NOT NULL,
            timestamp TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_channels_team ON channels (team);
        CREATE INDEX IF NOT EXISTS idx_users_team ON users (team);
        ";

        self.connection.execute(query)?;
        Ok(())
    }
}

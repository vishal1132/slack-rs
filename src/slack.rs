use crate::cache::cache::Cache;
use crate::decryptor::UnixCookieDecryptor;
use crate::model::domain::User;
use colored::Colorize;
use fake::Fake;
use keyring::Entry;
use rand::Rng;
use regex::Regex;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

pub struct Slack {
    auth: Option<Auth>,
    client: Client,
    team: Option<String>,
    cache: Box<dyn Cache>,
    user_map: HashMap<String, String>,
}

pub enum Auth {
    Cookie(CookieAuth),
}

pub struct CookieAuth {
    cookie: String,
    token: Option<String>,
}

pub fn new(team: &str, cache: Box<dyn Cache>) -> Result<Slack, Box<dyn Error>> {
    let mut s = Slack {
        auth: None,
        client: Client::new(),
        team: Some(team.into()),
        cache: cache,
        user_map: HashMap::new(),
    };
    s.auth(team)?;
    Ok(s)
}

impl Slack {
    fn password() -> Result<Vec<u8>, Box<dyn Error>> {
        let account_names = vec!["Slack Key", "Slack", "Slack App Store Key"];

        for account_name in account_names {
            match Slack::cookie_password_from_keychain(account_name) {
                Ok(password) => return Ok(password),
                _ => continue,
            }
        }
        Err("No password found for any account name".into())
    }

    fn slack_config_dir() -> Option<String> {
        let home = std::env::var("HOME").ok()?;
        let first = format!("{home}/Library/Application Support/Slack");
        let second=format!("{home}/Library/Containers/com.tinyspeck.slackmacgap/Data/Library/Application Support/Slack");
        if Path::new(&first).exists() {
            Some(first)
        } else {
            Some(second)
        }
    }

    fn cookie_password_from_keychain(account_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let service = "Slack Safe Storage";
        let keyring = Entry::new(service, account_name)?;
        let password = keyring.get_password()?;
        Ok(password.into_bytes())
    }

    fn format_cookie(key: &str, value: &str) -> String {
        format!("{}={}", key, value)
    }

    fn get_cookie_value_encrypted_value() -> Result<(String, Vec<u8>), Box<dyn Error>> {
        let cookie_dbpath: String;
        match Slack::slack_config_dir() {
            Some(path) => {
                cookie_dbpath = format!("{}/Cookies", path);
            }
            None => {
                return Err("Could not find slack config dir".into());
            }
        }

        let connection = sqlite::open(cookie_dbpath)?;
        let query = r#"SELECT value, encrypted_value FROM cookies where host_key=".slack.com" AND name="d""#;
        let mut statement = connection.prepare(query)?;

        let mut value: Option<String> = None;
        let mut encrypted_value: Option<Vec<u8>> = None;

        if let sqlite::State::Row = statement.next()? {
            // Assign values if a row is found
            value = Some(statement.read::<String, _>(0)?);
            encrypted_value = Some(statement.read::<Vec<u8>, _>(1)?);
        }
        let mut encrypted_value = encrypted_value.unwrap();
        encrypted_value = encrypted_value[3..].to_vec(); // to remove the version bytes.
        Ok((value.unwrap(), encrypted_value))
    }

    fn get_channel(&self, channel_name: &str, channel_id: &str) -> String {
        if !channel_name.is_empty() {
            return channel_name.to_string();
        }
        let channel = self
            .cache
            .get_channel(self.team.as_ref().unwrap(), channel_id)
            .unwrap_or_else(|| crate::model::domain::Channel {
                id: channel_id.to_string(),
                name: channel_id.to_string(),
                is_channel: true,
            });
        channel.name
    }

    //auth currently only supports cookie auth
    fn auth(&mut self, team: &str) -> Result<(), Box<dyn Error>> {
        let (_value, encrypted_value) = Slack::get_cookie_value_encrypted_value()?;

        let password = Slack::password()?;

        let cookie =
            String::from_utf8(UnixCookieDecryptor::new(1003).decrypt(encrypted_value, &password)?)
                .unwrap();

        log::debug!("Cookie: {}", cookie);

        // add cookie
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", Slack::format_cookie("d", &cookie).parse()?);

        let res = self
            .client
            .get(format!("https://{team}.slack.com").as_str())
            .headers(headers)
            .send()?;

        let response = res.text()?;
        let re = Regex::new(r#""api_token":"([^"]+)""#).unwrap();
        let token = re.captures(&response).unwrap().get(1).unwrap().as_str();

        log::debug!("Token: {}", token);

        self.auth = Some(Auth::Cookie(CookieAuth {
            cookie,
            token: Some(token.into()),
        }));
        Ok(())
    }

    pub fn api<T>(
        &self,
        path: &str,
        params: HashMap<&str, &str>,
        use_team_name: bool,
    ) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let url = if use_team_name {
            format!(
                "https://{}.slack.com/api/{}",
                self.team.as_ref().unwrap(),
                path
            )
        } else {
            format!("https://slack.com/api/{}", path)
        };
        let mut url = url.parse::<url::Url>().unwrap();
        params.iter().for_each(|(k, v)| {
            url.query_pairs_mut().append_pair(k, v);
        });
        let mut headers = HeaderMap::new();
        let auth = self.auth.as_ref().unwrap();
        match auth {
            Auth::Cookie(cookie_auth) => {
                headers.insert(
                    "Cookie",
                    Slack::format_cookie("d", &cookie_auth.cookie)
                        .parse()
                        .unwrap(),
                );
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", cookie_auth.token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
                headers.insert(
                    "Content-Type",
                    "application/json; charset=utf-8".parse().unwrap(),
                );
            }
        }

        let res = self
            .client
            .get(url.as_ref())
            .headers(headers)
            .send()
            .unwrap();
        // calculate the size of response
        let body_bytes = res.bytes()?;
        let response_size = body_bytes.len();
        log::debug!("Response size: {}", response_size);
        let parsed_response: T = serde_json::from_slice::<T>(&body_bytes)?;

        Ok(parsed_response)
    }

    pub fn generate_random_name() -> String {
        use fake::faker::name::raw::Name;
        match rand::thread_rng().gen_range(1..5) {
            1 => return Name(fake::locales::FR_FR).fake(),
            2 => return Name(fake::locales::PT_BR).fake(),
            _ => return Name(fake::locales::EN).fake(),
            // 4=>{return Name(fake::locales::ZH_CN).fake()},
            // _=>{return Name(fake::locales::AR_SA).fake()},
        }
    }

    pub fn sync(&self) {
        self.sync_channels();
        self.sync_users();
    }

    pub fn sync_users(&self) {
        use crate::model::domain::User;
        use crate::model::users as model;
        let users = self
            .api::<model::Root>("users.list", collection! {}, true)
            .unwrap();
        log::debug!("syncing users {:?}", users);
        let dom_users: Vec<User> = users
            .members
            .iter()
            .map(|m| User {
                id: m.id.clone(),
                name: m.name.clone(),
            })
            .collect();
        self.cache
            .sync_users(self.team.as_ref().unwrap(), dom_users)
            .unwrap();
    }

    pub fn sync_channels(&self) {
        use crate::model::channels as model;
        use crate::model::domain::Channel;

        let channels = self
            .api::<model::Root>(
                "conversations.list",
                // collection! {"types"=>"public_channel,private_channel,mpim,im"},
                collection! {"types"=>"public_channel,private_channel", "limit"=>"1000"},
                true,
            )
            .unwrap();

        log::debug!("syncing channels {:?}", channels);

        let dom_channels: Vec<Channel> = channels
            .channels
            .iter()
            .map(|c| crate::model::domain::Channel {
                id: c.id.clone(),
                name: c.name.clone(),
                is_channel: c.is_channel,
            })
            .collect();

        self.cache
            .sync_channels(self.team.as_ref().unwrap(), dom_channels)
            .unwrap();
    }

    pub fn search(&mut self, keyword: &str, count: u32) {
        use crate::model::search as model;
        let items = self
            .api::<model::Root>(
                "search.messages",
                collection! {"query"=> keyword, "count"=> &count.to_string()},
                true,
            )
            .unwrap();

        items
            .messages
            .matches
            .into_iter()
            .filter(|m| !m.channel.is_mpim)
            .for_each(|m| {
                let formatted_text = self.format_text(m.text);
                let user_name = self.get_user_name(m.user);
                println!(
                    "{} in #{}: {}\n",
                    user_name.italic().bold().yellow(),
                    self.get_channel(m.channel.name.as_str(), m.channel.id.as_str())
                        .italic()
                        .bold()
                        .green(),
                    self.highlight_keyword(formatted_text.as_ref(), keyword)
                );
            });
    }

    pub fn thread(&mut self, channel: &str, ts: &str) -> Result<(), Box<dyn Error>> {
        use crate::model::replies as model;
        let res = self.api::<model::Root>(
            "conversations.replies",
            collection! {"channel"=> channel, "ts"=>ts, "limit"=>"100", "inclusive"=>"true"},
            true,
        )?;
        self.print_messages(res.messages.as_ref());
        Ok(())
    }

    pub fn highlight_keyword(&mut self, text: &str, keyword: &str) -> String {
        let re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(keyword))).unwrap();
        let text = re.replace_all(text, |caps: &regex::Captures| {
            caps.get(0).unwrap().as_str().red().bold().to_string()
        });
        text.to_string()
    }

    fn format_text(&mut self, text: String) -> String {
        let new_line_replacer = Regex::new(r"\n{2,}").unwrap();
        let user_placeholder_replacer = Regex::new(r"<@([^>]+)>").unwrap();
        let gt_replacer = Regex::new(r"&gt;.*\n").unwrap();
        let text = new_line_replacer.replace_all(&text, "\n");
        let text = gt_replacer.replace_all(&text, |caps: &regex::Captures| {
            caps.get(0)
                .unwrap()
                .as_str()
                .replace("&gt;", "|")
                .blue()
                .bold()
                .italic()
                .to_string()
            // caps.get(0).unwrap().as_str().replace("&gt;", "|").black().on_white().bold().italic().to_string()
        });
        let text = user_placeholder_replacer.replace_all(&text, |caps: &regex::Captures| {
            let user_id = caps.get(1).unwrap().as_str().to_string();
            let user_name = self.get_user_name(user_id);
            format!("@{}", user_name.italic().bold().yellow())
        });
        text.to_string()
    }

    fn get_user_name(&mut self, user_id: String) -> String {
        let user = self
            .cache
            .get_user(self.team.as_ref().unwrap(), &user_id)
            .unwrap_or_else(|| User {
                name: self
                    .user_map
                    .entry(user_id.clone())
                    .or_insert_with(|| Slack::generate_random_name())
                    .to_string(),
                id: user_id.clone(),
            });
        user.name
    }

    fn print_messages(&mut self, messages: &Vec<crate::model::message::Message>) {
        messages.iter().for_each(move |m| {
            let user = m.user.clone();
            let user_name = self.get_user_name(user.clone());
            let text = self.format_text(m.text.clone());
            println!("{}: {}\n", user_name.italic().bold().yellow(), text);
        });
    }

    pub fn read(&mut self, channel: &str, start_time: &str) -> Result<(), Box<dyn Error>> {
        use crate::model::conversations as model;
        let res = self.api::<model::Root>(
            "conversations.history",
            collection! {"channel"=> channel, "ts"=>start_time,"limit"=>"100", "inclusive"=>"true"},
            true,
        )?;
        self.print_messages(&res.messages);

        Ok(())
    }
}

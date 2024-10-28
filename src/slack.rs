use crate::cache::cache::Cache;
use crate::decryptor::UnixCookieDecryptor;
use crate::model;
use keyring::Entry;
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
    // cache: Cache,
}

pub enum Auth {
    Cookie(CookieAuth),
}

pub struct CookieAuth {
    cookie: String,
    token: Option<String>,
}

pub fn new(team: &str) -> Result<Slack, Box<dyn Error>> {
    let mut s = Slack {
        auth: None,
        client: Client::new(),
        team: Some(team.into()),
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

    fn slackConfigDir() -> Option<String> {
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
        match Slack::slackConfigDir() {
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

    //auth currently only supports cookie auth
    fn auth(&mut self, team: &str) -> Result<(), Box<dyn Error>> {
        let (_value, encrypted_value) = Slack::get_cookie_value_encrypted_value()?;

        let password = Slack::password()?;

        let cookie =
            String::from_utf8(UnixCookieDecryptor::new(1003).decrypt(encrypted_value, &password)?)
                .unwrap();

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

        self.auth = Some(Auth::Cookie(CookieAuth {
            cookie,
            token: Some(token.into()),
        }));
        Ok(())
    }

    pub fn api<T>(&self, path: &str, params: HashMap<&str, &str>) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let url = format!(
            "https://{}.slack.com/api/{}",
            self.team.as_ref().unwrap(),
            path
        );
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
        println!("Response size: {}", response_size);
        // Deserialize the response into type `T`
        let parsed_response: T = serde_json::from_slice(&body_bytes)?;
        // let parsed_response = res.json::<T>()?; // Deserialize response into T

        Ok(parsed_response)
    }

    pub fn thread(&self, channel: &str, ts: &str) -> Result<(), Box<dyn Error>> {
        use crate::model::replies as model;
        let res = self.api::<model::Root>(
            "conversations.replies",
            collection! {"channel"=> channel, "ts"=>ts, "limit"=>"100", "inclusive"=>"true"},
        )?;
        res.messages.iter().for_each(|m| {
            println!("{}: {}", m.user, m.text);
        });
        Ok(())
    }

    pub fn read(&self, channel: &str, start_time: &str) -> Result<(), Box<dyn Error>> {
        use crate::model::conversations as model;
        let res = self.api::<model::Root>(
            "conversations.history",
            collection! {"channel"=> channel, "ts"=>start_time,"limit"=>"100", "inclusive"=>"true"},
        )?;
        res.messages.iter().for_each(|m| {
            println!("{}: {}", m.user, m.text);
        });

        // println!("{:?}", res);
        Ok(())
    }

    fn users(&self) {}
}

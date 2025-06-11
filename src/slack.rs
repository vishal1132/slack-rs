use crate::cache::cache::Cache;
use crate::decryptor::UnixCookieDecryptor;
use crate::model::domain::User;
use colored::Colorize;
use serde_json::Value;
use core::f64;
use fake::Fake;
use itertools::Itertools;
use keyring::Entry;
use rand::Rng;
use regex::Regex;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::de::DeserializeOwned;
use std::collections::{hash_map, HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use crate::model::message::Message;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

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

// #[derive(Clone)]
pub struct Slack {
    auth: Option<Auth>,
    client: Client,
    team: Option<String>,
    cache: Box<dyn Cache>,
    user_map: HashMap<String, String>,
}

#[derive(Clone)]
pub enum Auth {
    Cookie(CookieAuth),
}

#[derive(Clone)]
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

        log::info!("Cookie: {}", cookie);

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

        log::info!("Token: {}", token);

        self.auth = Some(Auth::Cookie(CookieAuth {
            cookie,
            token: Some(token.into()),
        }));
        Ok(())
    }

    pub fn post_api<T, B>(
        &self,
        path: &str,
        params: HashMap<&str, &str>,
        body: B,
        use_team_name: bool,
    ) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        // URL construction (same as api<T>)
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
        
        // Add query parameters (same as api<T>)
        params.iter().for_each(|(k, v)| {
            url.query_pairs_mut().append_pair(k, v);
        });
        
        // Headers setup (same as api<T>)
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
        
        // Log URL (same as api<T>)
        log::info!("url: {}", url);
        
        // Serialize body to JSON
        let json_body = serde_json::to_string(&body)?;
        
        // Send POST request with body (different from api<T>)
        let res = self
            .client
            .post(url.as_ref())
            .headers(headers)
            .body(json_body)
            .send()
            .unwrap();
        
        // Process response (same as api<T>)
        let body_bytes = res.bytes()?;
        let response_size = body_bytes.len();
        log::info!("Response size: {}", response_size);
        log::debug!("Response: {:?}", body_bytes);
    
        let parsed_response: T = serde_json::from_slice::<T>(&body_bytes)?;
    
        Ok(parsed_response)
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
        log::info!("url: {}", url);
        let res = self
            .client
            .get(url.as_ref())
            .headers(headers)
            .send()
            .unwrap();
        // calculate the size of response
        let body_bytes = res.bytes()?;
        let response_size = body_bytes.len();
        log::info!("Response size: {}", response_size);
        log::debug!("Response: {:?}", body_bytes);

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

    pub fn send(&self, msg: String){
        let mut body: HashMap<&str, &str>=HashMap::new();
        body.insert("channel", "D084TR3F18X");
        body.insert("text", msg.as_str());
        println!("sending message {:?}", body);
        let res = self
            .post_api::<HashMap<String, Value>, HashMap<&str,&str>>(
                "chat.postMessage",
                collection! {},
                body,
                true,
            )
            .unwrap();
        log::info!("send message {:?}", res);
    }

    pub fn sync_users(&self) {
        use crate::model::domain::User;
        use crate::model::users as model;
        let users = self
            .api::<model::Root>("users.list", collection! {}, true)
            .unwrap();
        log::info!("syncing users {:?}", users);
        let dom_users: Vec<User> = users
            .members
            .iter()
            .map(|m| User {
                id: m.id.clone(),
                name: m.name.clone(),
                email: m.profile.email.clone()
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

        log::info!("syncing channels {:?}", channels);

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
            .sorted_by(|a, b| {
                let a_ts = a.ts.parse::<f64>().unwrap_or(f64::MIN);
                let b_ts = b.ts.parse::<f64>().unwrap_or(f64::MIN);
                b_ts.partial_cmp(&a_ts).unwrap()
            })
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
        let messages=self.thread_messages(channel, ts)?;
        self.print_messages(messages.as_ref());
        Ok(())
    }

    fn thread_messages(&mut self, channel: &str, ts: &str) -> Result<Vec<Message>, Box<dyn Error>>{
        use crate::model::replies as model;
        let res = self.api::<model::Root>(
            "conversations.replies",
            collection! {"channel"=> channel, "ts"=>ts, "limit"=>"100", "inclusive"=>"true"},
            true,
        )?;
        // self.print_messages(res.messages.as_ref());
        Ok(res.messages)
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
            .unwrap_or_else(|| {
                let name=self
                    .user_map
                    .entry(user_id.clone())
                    .or_insert_with(|| Slack::generate_random_name())
                    .to_string();
                let email=format!("{name}@gmail.com");
                User {
                    name: name,
                    id: user_id.clone(),
                    email: email
                }
            });
        user.name
    }

    fn print_messages(&mut self, messages: &Vec<crate::model::message::Message>) {
        messages
            .iter()
            .filter(|m| !m.text.is_empty())
            .sorted_by(|a, b| {
                let a_ts = a.ts.parse::<f64>().unwrap_or(f64::MIN);
                let b_ts = b.ts.parse::<f64>().unwrap_or(f64::MIN);
                a_ts.partial_cmp(&b_ts).unwrap()
            })
            .for_each(move |m| {
                let user = m.user.clone();
                let user_name = self.get_user_name(user.clone());
                let text = self.format_text(m.text.clone());
                println!("{}: {}\n", user_name.italic().bold().yellow(), text);
            });
    }

    pub fn read_threaded(&mut self,
        channel: &str,
        start_time: &str,
        count: u32,
    )-> Result<(), Box<dyn Error>> {
        let mut messages=Vec::new();
        let mut cursor_empty_already = false;
        let mut start_time=start_time.to_string().clone();
        self.read_paginated(&mut messages,channel, &mut start_time, count, true, "", &mut cursor_empty_already)?;
        let mut threaded_ts_set: HashSet<String>=HashSet::new();
        messages.iter().filter(|m|m.thread_ts.is_some() || (m.thread_ts.is_none() && m.ts!="") && !m.text.contains("has joined the channel")).for_each(|m|{
            if m.thread_ts.is_none(){
                threaded_ts_set.insert(m.ts.clone());
            }else{
                threaded_ts_set.insert(m.thread_ts.clone().unwrap());
            }
        });
        println!("total unique threads {}",threaded_ts_set.len());
        let mut thread_messages: Vec<Vec<Message>>=Vec::new();
        
        // threaded_ts_set.iter().for_each(|ts|{
        //     let mut thread=Vec::new();
        //     messages.iter().filter(|m| m.thread_ts.is_some() && m.thread_ts.clone().unwrap()==*ts && m.ts!=*ts).for_each(|m|{
        //         thread.push(m.clone());
        //     });
        //     thread_messages.push(thread);
        // });
        // let thread_messages = Arc::new(Mutex::new(Vec::new()));
        // let pool = ThreadPool::new(10);

        // for ts in threaded_ts_set {
        //     let channel = channel.to_string();
        //     let thread_messages = Arc::clone(&thread_messages);
        //     let self_clone = self.clone(); // Assuming Slack implements Clone

        //     pool.execute(move || {
        //         let thread = self_clone.thread_messages(&channel, &ts).unwrap();
        //         let mut thread_messages = thread_messages.lock().unwrap();
        //         thread_messages.push(thread);
        //     });
        // }
        // pool.join();
        // let thread_messages = Arc::try_unwrap(thread_messages).unwrap().into_inner().unwrap();

        threaded_ts_set.iter().for_each(|ts|{
            let thread=self.thread_messages(channel, ts).unwrap();
            thread_messages.push(thread);
        });
        thread_messages.iter().for_each(|thread|{
            if thread.len()<=1{
                return;
            }
            self.print_messages(thread);
            println!("\n\n\n\n");
        });
        Ok(())

    }

    pub fn read(
        &mut self,
        channel: &str,
        start_time: &str,
        count: u32,
    ) -> Result<(), Box<dyn Error>> {
        // use crate::model::conversations as model;
        // let res = self.api::<model::Root>(
        //     "conversations.history",

        //     collection! {"channel"=> channel, "ts"=>start_time,"limit"=> format!("{}", count).as_ref(), "inclusive"=>"true"},
        //     true,
        // )?;
        // println!("empty thread ts {}",res.messages.iter().filter(|m|m.thread_ts == None).count());
        // // self.print_messages(&res.messages);
        // println!("cursor {}",res.response_metadata.next_cursor);
        let mut messages=Vec::new();
        let mut cursor_empty_already = false;
        let mut start_time=start_time.to_string().clone();
        self.read_paginated(&mut messages,channel, &mut start_time, count, true, "", &mut cursor_empty_already)?;
        self.print_messages(&messages);

        log::info!("Total Messages: {}", messages.len());
        Ok(())
    }

    fn get_max_ts(&mut self, messages: &Vec<crate::model::message::Message>) -> String{
        messages.iter().max_by(|a,b|{
            let a_ts = a.ts.parse::<f64>().unwrap_or(f64::MIN);
            let b_ts = b.ts.parse::<f64>().unwrap_or(f64::MIN);
            a_ts.partial_cmp(&b_ts).unwrap()
        }).map(|m|m.ts.clone()).unwrap_or("".to_string())
    }

    fn read_paginated(
        &mut self,
        messages: &mut Vec<crate::model::message::Message>,
        channel: &str,
        start_time: &mut String,
        count: u32,
        first_time: bool,
        cursor: &str,
        cursor_empty_already: &mut bool,
    ) -> Result<(), Box<dyn Error>> {
        log::info!(
            "current cursor-> {}, first time-> {}, count-> {}, start_time-> {}, cursor_empty_already-> {}",
            cursor,
            first_time,
            count,
            start_time,
            cursor_empty_already
        );
        if cursor == "" && !first_time && *cursor_empty_already{
            // println!("early return");
            return Ok(());
        }
        use crate::model::conversations as model;
        let count_str = format!("{}", count);
        let res = self.api::<model::Root>(
            "conversations.history",
            if cursor!=""{
                collection! {"channel"=> channel, "limit"=> count_str.as_ref(), "cursor"=>cursor}
            } else{
                 collection! {"channel"=> channel, "oldest"=>start_time,"limit"=> count_str.as_ref()}
            },
            false,
        )?;
        if res.messages.len()==0{
            *cursor_empty_already=true;
            return Ok(());
        }
        if res.response_metadata.next_cursor==""{
            if self.get_max_ts(messages) == self.get_max_ts(res.messages.as_ref()){
                *cursor_empty_already=true;
            }else{
                *start_time=self.get_max_ts(messages);
            }
        }
        // println!("next cursor-> {}", res.response_metadata.next_cursor);
        log::info!("next cursor-> {}", res.response_metadata.next_cursor);
        log::info!(
            "empty thread ts {}",
            res.messages.iter().filter(|m| m.thread_ts == None && m.ts=="").count()
        );
        messages.extend(res.messages);
        self.read_paginated(
            messages,
            channel,
            start_time,
            count,
            false,
            res.response_metadata.next_cursor.as_ref(),
            cursor_empty_already,
        )?;
        Ok(())
    }
}

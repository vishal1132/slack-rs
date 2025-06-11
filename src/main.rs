mod cache;
mod cli;
mod decryptor;
mod model;
mod slack;

use cache::sqlite::InMemoryCache as db;
use clap::Parser;
use cli::{Cli, SubCommand};
use std::{error::Error, ops::Sub};

fn parse_url(arg: &str) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let url = url::Url::parse(arg)?;
    let team = url
        .host_str()
        .map(|x| x.split('.'))
        .and_then(|mut x| x.next())
        .unwrap()
        .to_string();

    let mut path_segments = url.path_segments().expect("no path segments in URL");
    let channel = path_segments.nth(1).expect("no channel").to_string();

    let start_time = path_segments
        .nth(0)
        .and_then(|x| x.strip_prefix("p"))
        .map(|x| x.to_string())
        .unwrap();
    Ok((team, channel, start_time))
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut log_filter_level = log::LevelFilter::Warn;
    if cli.verbose {
        log_filter_level = log::LevelFilter::Info;
    }
    if cli.debug {
        log_filter_level = log::LevelFilter::Debug;
    }
    env_logger::Builder::new()
        .filter_level(log_filter_level)
        .init();
    let db = db::new()?;

    match cli.subcmd {
        SubCommand::Read { ref arg, .. } | SubCommand::ReadThreaded { ref arg, .. }| SubCommand::Thread { ref arg, .. } => {
            let (team, channel, start_time) = parse_url(arg)?;
            let mut slack_client = slack::new(team.as_ref(), db)?;
            match cli.subcmd {
                SubCommand::Read { count, .. } => {
                    slack_client.read(&channel, &start_time, count)?
                }
                SubCommand::ReadThreaded { count, .. } => {
                    slack_client.read_threaded(&channel, &start_time, count)?
                }
                SubCommand::Thread { .. } => slack_client.thread(&channel, &start_time)?,
                _ => {}
            }
        }
        SubCommand::Search {
            keyword,
            team,
            count,
        } => {
            let mut slack_client = slack::new(team.as_ref(), db)?;
            slack_client.search(&keyword, count);
        }
        SubCommand::Sync { team } => {
            let slack_client = slack::new(team.as_ref(), db)?;
            slack_client.sync();
        },
        SubCommand::Send{team, msg}=> { 
            let slack_client = slack::new(team.as_ref(), db)?;
            slack_client.send(msg);
         }
    }

    Ok(())
}

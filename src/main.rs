mod cache;
mod cli;
mod decryptor;
mod model;
mod slack;

use clap::Parser;
use cli::{Cli, SubCommand};
use std::error::Error;

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
    let mut log_filter_level = log::LevelFilter::Info;
    if cli.verbose {
        log_filter_level = log::LevelFilter::Debug;
    }
    env_logger::Builder::new()
        .filter_level(log_filter_level)
        .init();

    match cli.subcmd {
        SubCommand::Read { ref arg } | SubCommand::Thread { ref arg } => {
            let (team, channel, start_time) = parse_url(arg)?;
            let slack_client = slack::new(team.as_ref())?;
            match cli.subcmd {
                SubCommand::Read { .. } => slack_client.read(&channel, &start_time)?,
                SubCommand::Thread { .. } => slack_client.thread(&channel, &start_time)?,
            }
        }
    }

    Ok(())
}

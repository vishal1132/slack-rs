mod cache;
mod cli;
mod decryptor;
mod model;
mod slack;

use cache::sled;
use clap::Parser;
use cli::{cli as Cli, SubCommand};
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

    let (team, channel, start_time) = match cli.subcmd {
        SubCommand::Read { ref arg } | SubCommand::Thread { ref arg } => parse_url(arg)?,
    };

    let slack_client = slack::new(team.as_ref())?;
    match cli.subcmd {
        SubCommand::Read { .. } => slack_client.read(&channel, &start_time)?,
        SubCommand::Thread { .. } => slack_client.thread(&channel, &start_time)?,
    };

    Ok(())
}

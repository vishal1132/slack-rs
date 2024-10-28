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
    let team = url.host_str().unwrap().split('.').next().expect("no team").to_string();
    let mut path_segments = url.path_segments().expect("no path segments in URL");
    let channel = path_segments.nth(1).expect("no channel").to_string();
    let start_time = path_segments
        .nth(0)
        .expect("no timestamp")
        .strip_prefix('p')
        .expect("no prefix")
        .to_string();
    Ok((team, channel, start_time))
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Read{arg}=> {
            let (team, channel, start_time)=parse_url(&arg)?;
            slack::new(team.as_ref())?.read(&channel, &start_time)?;
        }
        SubCommand::Thread{arg} => {
            let (team, channel, start_time)=parse_url(&arg)?;
            slack::new(team.as_ref())?.thread(&channel, &start_time)?;
        }
    }
    
    Ok(())
}

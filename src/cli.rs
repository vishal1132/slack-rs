use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    #[clap(short, long)]
    #[clap(default_value = "false")]
    pub verbose: bool,
    #[clap(short, long)]
    #[clap(default_value = "false")]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Read {
        arg: String,
        #[clap(short, long)]
        #[clap(default_value = "10")]
        count: u32,
    },
    ReadThreaded {
        arg: String,
        #[clap(short, long)]
        #[clap(default_value = "10")]
        count: u32,
    },
    Thread {
        arg: String,
        #[clap(short, long)]
        #[clap(default_value = "10")]
        count: u32,
    },
    Search {
        #[clap(short, long)]
        keyword: String,
        #[clap(short, long)]
        team: String,
        #[clap(short, long)]
        #[clap(default_value = "10")]
        count: u32,
    },
    Sync {
        #[clap(short, long)]
        team: String,
    },
    Send{
        #[clap(short, long)]
        team: String,
        #[clap(short, long)]
        msg: String,
    }
}

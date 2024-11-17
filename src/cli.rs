use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    #[clap(short, long)]
    #[clap(default_value = "false")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Read {
        arg: String,
    },
    Thread {
        arg: String,
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
}

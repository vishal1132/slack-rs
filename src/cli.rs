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
    Read { arg: String },
    Thread { arg: String },
}

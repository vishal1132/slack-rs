use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Read { arg: String },
    Thread { arg: String },
}

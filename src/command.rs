pub mod debug;
pub mod run;
pub mod test;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
     #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Run {},
    Test {},
    Debug{},
}

pub fn parse() -> Cli {
    Cli::parse()
}
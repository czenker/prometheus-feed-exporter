pub mod debug;
pub mod run;
pub mod test;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
     #[command(subcommand)]
    pub command: Command,
     #[arg(short, long, default_value="example/settings.yaml")]
    pub settings_file: String,
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
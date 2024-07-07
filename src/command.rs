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
    /// Fetch configured feeds and check for significant entries
    Run {},
    /// run unit tests against filter rules
    Test{
        /// path to test file
        #[arg()]
        file: String,
    },
    /// show debug information on entries
    Debug{
        /// id of the feed to show debug information for
        #[arg()]
        feed: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
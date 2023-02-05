use clap;
use clap::{CommandFactory, Parser, Subcommand};

use crate::config::Config;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Listen {
        #[clap(long, value_parser, value_name = "FILE")]
        config_file: PathBuf,
    },
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn execute() -> Option<Config> {
        let parsed_cli = Self::parse();
        match &parsed_cli.command {
            Some(Command::Listen { config_file }) => Config::new(config_file).ok(),
            None => {
                Self::command().print_help().unwrap();
                None
            }
        }
    }
}

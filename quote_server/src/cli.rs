use clap;
use clap::{CommandFactory, Parser, Subcommand};
use log::info;
use quote_lib::run_quotes_service;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Listen {
        #[clap(long, value_parser, value_name = "FILE")]
        quotes_file: PathBuf,
        #[clap(long, value_parser, value_name = "PORT")]
        port: u16,
        #[clap(long, value_parser, value_name = "CRAP_PWD")]
        crap_password: String,
    },
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn execute() {
        let parsed_cli = Self::parse();
        info!("Current dir: {:?}", std::env::current_dir());

        match &parsed_cli.command {
            Some(Command::Listen {
                quotes_file,
                port,
                crap_password,
            }) => {
                info!("Quotes source path: {:?}", quotes_file);
                run_quotes_service(quotes_file, *port, crap_password);
            }
            None => {
                Self::command().print_help().unwrap();
            }
        }
    }
}

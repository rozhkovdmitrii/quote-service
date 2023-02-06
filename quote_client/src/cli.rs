use super::client;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    GetQuote {
        #[clap(long, value_parser, value_name = "HOST")]
        host: String,
        #[clap(long, value_parser, value_name = "PORT")]
        port: u16,
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
        match &parsed_cli.command {
            Some(Command::GetQuote { host, port }) => {
                client::run_quotes_client(host, port);
            }
            None => {
                Self::command().print_help().unwrap();
            }
        }
    }
}

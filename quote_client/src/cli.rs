use super::client;

use clap;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    GetQuote {
        #[clap(long, value_parser, value_name = "IPV4")]
        ipaddr: String,
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
        match &parsed_cli.command {
            Some(Command::GetQuote {
                ipaddr,
                port,
                crap_password,
            }) => {
                client::run_quotes_client(ipaddr, port, crap_password);
            }
            None => {
                Self::command().print_help().unwrap();
            }
        }
    }
}

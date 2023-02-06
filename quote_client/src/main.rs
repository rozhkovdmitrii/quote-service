mod cli;
mod client;
mod config;

const LOG_CFG: &str = "config/log4rs.yml";

fn main() {
    if let Err(error) = log4rs::init_file(LOG_CFG, Default::default()) {
        println!("Failed to get log from: {}, error: {}", LOG_CFG, error);
        return;
    };
    cli::Cli::execute();
}

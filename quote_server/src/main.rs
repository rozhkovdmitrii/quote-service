mod cli;
mod config;
mod quotes_storage;
mod service;

use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

const LOG_CFG: &str = "config/log4rs.yml";

fn main() {
    if let Err(error) = log4rs::init_file(LOG_CFG, Default::default()) {
        println!("Failed to get log from: {}, error: {}", LOG_CFG, error);
        return;
    };
    if let Some(config) = cli::Cli::execute() {
        run_quotes_service(config);
    };
}

#[tokio::main(flavor = "multi_thread")]
pub async fn run_quotes_service(config: config::Config) {
    info!("Start quotes service with config: {}", config);
    let quotes_storage =
        quotes_storage::QuotesStorageImpl::new(config.quotes_file(), config.skip_lines());
    let service = service::Service::new(config, Arc::new(Mutex::new(Box::new(quotes_storage))));
    service.run().await;
}

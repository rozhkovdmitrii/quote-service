mod cli;
mod config;
mod quotes_storage;
mod service;

use log::info;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use config::Config;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    if let Some(config) = cli::Cli::execute() {
        run_quotes_service(config);
    };
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
pub async fn run_quotes_service(config: Config) {
    info!("Initialization begin, read quotes from: {:?}", config.quotes_file());
    let quotes_storage = quotes_storage::QuotesStorageImpl::new();
    let service = service::Service::new(config, Arc::new(Mutex::new(Box::new(quotes_storage))));
    service.run().await;
}

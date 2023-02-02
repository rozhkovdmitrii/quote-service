mod data;
mod service;

use log::info;
use std::path::PathBuf;
use tokio;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
pub async fn run_quotes_service(quotes_file: &PathBuf, port: u16, crap_password: &String) {
    info!("Initialization begin, read quotes from: {:?}", quotes_file);
    let service = service::Service::new(port, crap_password);
    service.run().await;
}

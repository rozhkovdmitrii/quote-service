use super::quotes_storage::QuotesStorage;
use super::Config;
use log::{debug, error, info, warn};
use quote_lib::network::{read_u64, write_u64};
use quote_lib::pow::check_auth_and_pow;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct Service {
    nonce: Arc<Mutex<u64>>,
    config: Arc<Config>,
    quotes_storage: Arc<Mutex<Box<dyn QuotesStorage>>>,
}

impl Service {
    pub fn new(config: Config, quotes_storage: Arc<Mutex<Box<dyn QuotesStorage>>>) -> Self {
        Self {
            nonce: Arc::new(Mutex::new(0)),
            config: Arc::new(config),
            quotes_storage,
        }
    }

    pub async fn run(self) {
        let (ip, port) = ("0.0.0.0", self.config.port());
        let server = TcpListener::bind((ip, port)).await.expect("Failed to bind tcp server");
        info!("Bound tcp server: {}, {}", ip, port);
        loop {
            if let Ok((stream, addr)) = server.accept().await {
                info!("Successfully accepted new tcp stream: {}", addr.to_string());
                tokio::spawn(Self::handle_connection(
                    stream,
                    self.config.clone(),
                    self.nonce.clone(),
                    self.quotes_storage.clone(),
                ));
            } else {
                debug!("Failed to accept tcp stream.");
                continue;
            }
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        config: Arc<Config>,
        defended_nonce: Arc<Mutex<u64>>,
        quotes: Arc<Mutex<Box<dyn QuotesStorage>>>,
    ) {
        let nonce = Service::get_nonce(defended_nonce).await;
        Service::send_nonce(&mut stream, nonce).await;
        let (mut reader, mut writer) = stream.split();
        tokio::time::sleep(Duration::from_secs(5)).await;

        info!("Waiting for a bump that proves bot authentication and work");

        let bump_seed = read_u64(&mut reader).await;

        if let Err(error) = bump_seed {
            error!("Failed to read bump seed: {}", error);
            return;
        }
        let bump_seed = bump_seed.unwrap();

        let mut hash = [0; 32];
        reader.read(&mut hash).await.unwrap(); // TODO: не обработано !!!
        let hex_hash = hex::encode(hash);
        debug!("For nonce: {} - got bump seed: {}, and hash: {}", nonce, bump_seed, hex_hash);
        if !check_auth_and_pow(nonce, config.crap_password(), bump_seed, &hash) {
            warn!("Failed to check challenge response authentication");
            return;
        }
        info!("Authentication and proof of work checks passed");
        let mut quotes_guard = quotes.lock().await;
        let quote = quotes_guard.get_quote().await;
        writer.write(quote.as_bytes()).await.unwrap();
    }

    async fn send_nonce(tcp_stream: &mut TcpStream, nonce: u64) {
        let (_, mut writer) = tcp_stream.split();
        debug!("Send nonce into stream: {}", nonce);
        if let Ok(_) = write_u64(nonce, &mut writer).await {
            debug!("Nonce: {} - sent", nonce);
        };
    }

    async fn get_nonce(defended_nonce: Arc<Mutex<u64>>) -> u64 {
        loop {
            match defended_nonce.try_lock() {
                Ok(mut lock_guard) => {
                    *lock_guard += 1;
                    return *lock_guard;
                }
                Err(_) => tokio::time::sleep(Duration::from_secs(0)).await,
            }
        }
    }
}

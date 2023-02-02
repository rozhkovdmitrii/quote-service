use log::{debug, error, info};
use std::io::{Error, ErrorKind};
use tokio::net::{TcpListener, TcpStream};

use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::Mutex;

pub struct Service {
    nonce: Arc<Mutex<u64>>,
    port: u16,
    crap_password: RwLock<String>,
}

impl Service {
    pub fn new(port: u16, crap_password: &String) -> Self {
        Self {
            nonce: Arc::new(Mutex::new(0)),
            port,
            crap_password: RwLock::new(crap_password.clone()),
        }
    }

    pub async fn run(self) {
        let (ip, port) = ("0.0.0.0", self.port);
        let server = TcpListener::bind((ip, port)).await.expect("Failed to bind tcp server");
        info!("Bound tcp server: {}, {}", ip, port);
        loop {
            if let Ok((stream, addr)) = server.accept().await {
                info!("Successfully accepted new tcp stream: {}", addr.to_string());
                tokio::spawn(Self::handle_connection(stream, self.nonce.clone()));
            } else {
                debug!("Failed to accept tcp stream.");
                continue;
            }
        }
    }

    async fn handle_connection(mut stream: TcpStream, defended_nonce: Arc<Mutex<u64>>) {
        let nonce = Service::get_nonce(defended_nonce).await;
        Service::send_nonce(&mut stream, nonce).await;
        info!("Waiting for a bump that proves bot authentication and work");
    }

    async fn send_nonce(tcp_stream: &mut TcpStream, nonce: u64) {
        let (_, writer) = tcp_stream.split();
        debug!("Send nonce into stream: {}", nonce);
        let buff = bincode::encode_to_vec(nonce, bincode::config::standard())
            .expect("Failed to encode nonce to buff");
        loop {
            match writer.try_write(&buff) {
                Ok(bytes_sent) => {
                    debug!("Sent: {}", bytes_sent);
                    break;
                }
                Err(error) if error.kind() == ErrorKind::BrokenPipe => {
                    debug!("Connection has been closed");
                    break;
                }
                Err(error) => {
                    error!("Failed to write nonce to stream: {}", error);
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            }
        }
    }

    async fn read_hash(tcp_stream: &mut TcpStream) {
        let (reader, writer) = tcp_stream.split();
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

use super::config::Config;
use log::{debug, error, warn};
use quote_lib::{
    network::{read_u64, write_u64},
    pow::PowCalculator,
};
use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main(flavor = "multi_thread")]
pub async fn run_quotes_client(ipaddr: &String, port: &u16, crap_password: &String) {
    let client = QuotesClient::new(Config::new(ipaddr, port, crap_password));
    client.get_quote().await;
}

pub struct QuotesClient {
    config: Config,
    pow_calculator: PowCalculator,
}

impl QuotesClient {
    fn new(config: Config) -> QuotesClient {
        QuotesClient {
            config,
            pow_calculator: PowCalculator::new(),
        }
    }

    pub async fn get_quote(&self) {
        debug!("Connecting to: {}", self.config.srv_addr());
        let stream = TcpStream::connect(self.config.srv_addr()).await;
        if let Err(error) = stream {
            error!("Failed to connect: {} - {}", self.config.srv_addr(), error);
            return;
        }

        let mut stream = stream.unwrap();
        let (mut reader, mut writer) = stream.split();

        let nonce = read_u64(&mut reader).await.unwrap();
        let (bump_seed, hash) =
            self.pow_calculator.compute_bump_seed(nonce, &self.config.crap_password());
        debug!("Computed bump_seed: {}", bump_seed);
        write_u64(bump_seed, &mut writer).await.unwrap();
        match writer.write(&hash).await {
            Ok(bytes) => debug!("Hash that conforms to all seeds including secret sent: {}", bytes),
            Err(error) => {
                error!("Failed to send hash: {}", error);
                return;
            }
        }
        let mut quote = String::new();
        debug!("Waiting for quote");

        if let Err(error) = reader.readable().await {
            error!("Failed to get quote: {}", error);
        }
        match reader.read_to_string(&mut quote).await {
            Ok(0) => warn!("Connection has been closed remotely, secret is wrong"),
            Ok(_) => println!("Quote: {}", quote),
            Err(error) => error!("Failed to get quote: {}", error),
        }
    }
}

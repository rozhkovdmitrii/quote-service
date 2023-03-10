use super::config::Config;
use log::{debug, error, warn};
use quote_lib::{
    network::{read_u64, write_u64},
    pow::PowCalculator,
};
use std::env;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[tokio::main(flavor = "multi_thread")]
pub async fn run_quotes_client(host: &String, port: &u16) {
    let crap_secret = env::var("CRAP_SECRET");
    if let Err(error) = crap_secret {
        error!("Failed to get env 'CRAP_SECRET': {}", error);
        return;
    }
    let client = QuotesClient::new(Config::new(host, port, crap_secret.as_ref().unwrap()));
    client.get_quote().await.ok();
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

    pub async fn get_quote(&self) -> Result<(), ()> {
        debug!("Connecting to: {}", self.config.srv_addr());
        let mut stream = TcpStream::connect(self.config.srv_addr())
            .await
            .map_err(|error| error!("Failed to connect: {} - {}", self.config.srv_addr(), error))?;
        let (mut reader, mut writer) = stream.split();

        let nonce = read_u64(&mut reader)
            .await
            .map_err(|error| error!("Failed to send nonce: {}", error))?;
        debug!("Got nonce: {}", nonce);
        let (bump_seed, hash) =
            self.pow_calculator.compute_bump_seed(nonce, &self.config.crap_password());
        debug!("Computed bump_seed: {}", bump_seed);
        write_u64(bump_seed, &mut writer)
            .await
            .map_err(|error| error!("Failed to send bump_seed: {}", error))?;
        debug!("Hash that conforms to all seeds including secret: {}", &hex::encode(hash));

        let mut quote = String::new();
        debug!("Waiting for quote");

        match reader.read_to_string(&mut quote).await {
            Ok(0) => warn!("Connection has been closed remotely, secret is wrong"),
            Ok(_) => println!("Resulting quote: {}", quote),
            Err(error) => error!("Failed to get quote: {}", error),
        }
        Ok(())
    }
}

use bincode;
use log::{debug, error};
use std::fs::read;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::net::TcpStream;

#[tokio::main(flavor = "multi_thread")]
pub async fn run_quotes_client(ipaddr: &String, port: &u16, crap_password: &String) {
    debug!("Getting address from: \"{}:{}\"", ipaddr, port);
    let address = SocketAddr::from_str(format!("{}:{}", ipaddr, port).as_str())
        .expect("Failed to get socket address");
    QuotesClient::get_quote(address).await;
}

pub struct QuotesClient {}

impl QuotesClient {
    pub async fn get_quote(address: SocketAddr) {
        debug!("Connecting to: {}", address);
        let stream = TcpStream::connect(address).await;
        if let Err(error) = stream {
            error!("Failed to connect: {} - {}", address, error);
            return;
        }

        let stream = stream.unwrap();
        //TODO: this place is full of problems
        stream.readable().await;

        let mut buff = [0; 1024];
        match stream.try_read(&mut buff) {
            Ok(n) => {
                debug!("Read: {}", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => {}
        }
    }
}

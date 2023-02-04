use std::net::SocketAddr;
use std::str::FromStr;

pub struct Config {
    srv_addr: SocketAddr,
    crap_password: String,
}

impl Config {
    pub fn new(ipaddr: &String, port: &u16, crap_password: &String) -> Config {
        let srv_addr = SocketAddr::from_str(format!("{}:{}", ipaddr, port).as_str())
            .expect(format!("Failed to get socket address from: {}:{}", ipaddr, port).as_str());
        Config {
            srv_addr,
            crap_password: crap_password.clone(),
        }
    }

    pub fn srv_addr(&self) -> &SocketAddr {
        &self.srv_addr
    }

    pub fn crap_password(&self) -> &String {
        &self.crap_password
    }
}

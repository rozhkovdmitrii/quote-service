use std::path::PathBuf;

pub struct Config {
    port: u16,
    crap_password: String,
    quotes_file: PathBuf,
}

impl Config {
    pub fn new(port: &u16, crap_password: &String, quotes_file: &PathBuf) -> Config {
        Config {
            port: *port,
            crap_password: crap_password.clone(),
            quotes_file: quotes_file.clone(),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn crap_password(&self) -> &String {
        &self.crap_password
    }

    pub fn quotes_file(&self) -> &PathBuf {
        &self.quotes_file
    }
}

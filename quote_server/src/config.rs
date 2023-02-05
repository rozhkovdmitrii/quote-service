use log::{error, info};
use serde;
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Config {
    port: u16,
    quotes_file: PathBuf,
    skip_lines: usize,
    crap_secret: Option<String>,
}

impl Config {
    pub fn new(config_path: &PathBuf) -> Result<Config, ()> {
        info!("Read config from: {:?}", config_path);
        let mut file = File::open(config_path)
            .map_err(|error| error!("Cannot open config file: {}", error))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|error| error!("Failed to read config file: {}", error))?;

        let mut config: Config =
            serde_yaml::from_str(&buf).map_err(|error| error!("Cannot parse config: {}", error))?;
        config.crap_secret = Some(
            env::var("CRAP_SECRET")
                .map_err(|error| error!("Failed to get env 'CRAP_SECRET': {}", error))?,
        );
        Ok(config)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn crap_secret(&self) -> &String {
        self.crap_secret.as_ref().unwrap()
    }

    pub fn quotes_file(&self) -> &PathBuf {
        &self.quotes_file
    }

    pub fn skip_lines(&self) -> usize {
        self.skip_lines
    }
}

use log::{error, info};
use serde;
use serde_yaml;
use std::{
    env,
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
    path::PathBuf,
    time::Duration,
};

#[derive(serde::Deserialize)]
pub struct Config {
    port: u16,
    quotes_file: PathBuf,
    skip_lines: usize,
    bump_seed_timeout_millis: u64,
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

    pub fn bump_seed_timeout(&self) -> Duration {
        Duration::from_millis(self.bump_seed_timeout_millis)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "quotes_file: {:?}, port: {}, skip_lines: {}, bump_seed_timeout: {} ms.",
            self.quotes_file, self.port, self.skip_lines, self.bump_seed_timeout_millis
        )
    }
}

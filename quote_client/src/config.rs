pub struct Config {
    srv_addr: String,
    crap_password: String,
}

impl Config {
    pub fn new(host: &String, port: &u16, crap_password: &String) -> Config {
        Config {
            srv_addr: format!("{}:{}", host, port),
            crap_password: crap_password.clone(),
        }
    }

    pub fn srv_addr(&self) -> &String {
        &self.srv_addr
    }

    pub fn crap_password(&self) -> &String {
        &self.crap_password
    }
}

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub jwt: Jwt,
    pub log: Log,
}

lazy_static! {
    pub static ref CONFIG: Settings =
        Settings::new().expect("config can be loaded");
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(CONFIG_FILE_PATH))?;

        s.merge(Environment::with_prefix("vsts").separator("__"))?;

        s.try_into()
    }
}

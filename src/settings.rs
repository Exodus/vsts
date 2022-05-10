use config::{Config, ConfigError, Environment};
use serde::Deserialize;

// #[derive(Debug, Deserialize, Clone)]
// pub struct Log {
//     pub level: String,
// }

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
    // pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default("server.port", "3030")?
            .set_default("jwt.secret", "test")?
            // .set_default("log.level", "info")?
            .add_source(Environment::with_prefix("VSTS").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Settings = Settings::new().expect("config can be loaded");
}

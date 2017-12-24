use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Command {
    pub program: String,
    pub arguments: String
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: Server,
    pub command: Command
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }
}

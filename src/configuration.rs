use serde::Deserialize;

use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub token: String,
}


impl Configuration {
    pub fn new(path: Option<String>) -> Result<Self, ConfigError> {
        let mut configuration = Config::default();

        if let Some(path) = path {
            configuration.merge(File::with_name(&path[..]))?;
        }

        configuration.try_into()
    }
}


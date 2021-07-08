use std::path::{Path};
use serde::Deserialize;

use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub token: Option<String>,
    pub project_id: Option<i32>,
}


impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut configuration = Config::default();

        let config_dir = dirs::config_dir();
        if let Some(config_dir) = config_dir {
            let config_file = Path::new("samsonr/config.toml");
            let path = config_dir.join(config_file);
            configuration.merge(File::from(path))?;
        }

        configuration.try_into()
    }
}


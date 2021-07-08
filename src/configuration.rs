use std::path::{Path};
use serde::Deserialize;

use log::*;

use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub token: Option<String>,
    pub project_id: Option<i32>,
}


impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Initializing configuration");
        let mut configuration = Config::default();

        let config_dir = dirs::config_dir();
        if let Some(config_dir) = config_dir {
            debug!("Searching config in directory, directory={:?}", config_dir);

            let config_file = Path::new("samsonr/config.toml");
            let path = config_dir.join(config_file);
            debug!("Loading configuration from file, file={:?}", path);
            configuration.merge(File::from(path))?;
        }

        debug!("Returning configuration, configuration={:?}", configuration);
        configuration.try_into()
    }
}


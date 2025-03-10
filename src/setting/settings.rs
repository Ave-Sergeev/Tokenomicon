use anyhow::{Error, Result};
use config::Config;
use log::{Level, log};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Server {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new(location: &str) -> Result<Self, Error> {
        let mut builder = Config::builder();

        if Path::new(location).exists() {
            builder = builder.add_source(config::File::with_name(location));
        } else {
            log!(Level::Warn, "Configuration file not found");
        }

        let settings = builder.build()?.try_deserialize()?;

        Ok(settings)
    }

    pub fn json_pretty(&self) -> Result<String, Error> {
        let result = to_string_pretty(&self)?;

        Ok(result)
    }
}

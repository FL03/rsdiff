/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{
    try_collect_config_files, ConfigResult, Configurable, Logger, SerdeDisplay, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub mode: String,
    pub server: Server,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        Self {
            logger: Default::default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
            server: Server::new("0.0.0.0".to_string(), 8080),
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;

        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        if let Ok(port) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", port)?;
        };
        // Add in related environment variables
        builder = builder.add_source(Environment::default().separator("__").prefix(env!("CARGO_PKG_NAME").to_ascii_uppercase().as_str()));
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None);
        Self::build().unwrap_or(d)
    }
}

use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use toml;

/// Configuration for the server.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// The address and port that the server instance should bind to and listen on.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct ServerConfig {
    pub(crate) address: IpAddr,
    pub(crate) port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
        }
    }
}

/// Logging configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct LoggingConfig {
    pub(crate) level: LogLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Off,
        }
    }
}

/// Logging level.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum LogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "off")]
    Off,
}

/// Error cases for trying to read the configuration file.
#[derive(Debug, PartialEq)]
pub(crate) enum ConfigError {
    IOError,
    IllFormed,
}

/// Attempt to read a configuration file from the given path.
///
/// # Arguments
///
/// * `path` - The path to the configuration file in TOML format.
pub(crate) fn get_config(path: &str) -> Result<Config, ConfigError> {
    read_config_file_from_path(path).and_then(|s| parse_into_config(s.as_str()))
}

fn read_config_file_from_path(path: &str) -> Result<String, ConfigError> {
    let path = Path::new(path);
    info!("Trying to read config from {:?}", path);
    fs::read_to_string(path).map_err(|e| {
        error!("Failed to read {:#?}", path);
        error!("Error: {:#?}", e);
        ConfigError::IOError
    })
}

fn parse_into_config(raw: &str) -> Result<Config, ConfigError> {
    toml::from_str::<Config>(raw).map_err(|e| {
        error!("Illegal config format!");
        error!("Raw config: {:#?}", raw);
        error!("Error: {:#?}", e);
        ConfigError::IllFormed
    })
}

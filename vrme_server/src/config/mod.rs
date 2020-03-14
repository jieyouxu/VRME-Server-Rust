use config::{Config, ConfigError, Environment, File};
use log::{info, warn};
use serde::Deserialize;
use std::env;
use std::net::IpAddr;
use thiserror::Error;

/// Errors encountered when trying to determine the settings for the
/// application.
#[derive(Debug, Error)]
pub enum SettingsError {
    /// Failed to find settings file.
    #[error("settings file not found at: `{0}`")]
    NotFound(String),
    /// Failed to read configuration file due to IO errors.
    #[error("failed to read config due to IO error: `{0}`")]
    IOError(#[from] std::io::Error),
    /// Failed to parse settings file. Contains illegal syntax.
    #[error("invalid syntax: `{0}`")]
    InvalidSyntax(String),
    /// Other settings errors.
    #[error("settings error: `{0}`")]
    ConfigError(Box<dyn std::error::Error>),
}

impl std::convert::From<ConfigError> for SettingsError {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::NotFound(s) => Self::NotFound(s),
            e => Self::ConfigError(Box::new(e)),
        }
    }
}

/// The settings for the application.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
    pub server: ServerSettings,
}

/// Database settings.
#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub hostname: IpAddr,
    pub port: u16,
    pub database_name: String,
}

/// Logging settings.
#[derive(Debug, Deserialize)]
pub struct LoggingSettings {
    pub level: LoggingLevel,
}

/// Logging levels. The most specific is `LoggingLevel::Trace`, and the least
/// specific is `LoggingLevel::Error`.
///
/// # Logging in Production
///
/// **Production** environments should *not* use debug levels more specific
/// than `LoggingLevel::Info` as `LoggingLevel::Trace` and
/// `LoggingLevel::Debug` is permitted to log sensitive information such as
/// passwords and IPs to `stdout` or `stderr`.
#[derive(Debug, Deserialize)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Server settings.
#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    /// Which IP address the application server should bind to.
    hostname: IpAddr,
    /// Which port the application server should listen on.
    port: u16,
}

impl Settings {
    /// Construct new settings.
    ///
    /// # Settings Precedence
    ///
    /// By default, the order that settings are overriden is (in order of
    /// increasing precedence):
    ///
    /// 1. Settings file.
    ///     1. Default settings file located at `config/default`.
    ///     2. Based on `RUN_MODE` environment variable, we mix in:
    ///         - `RUN_MODE=production`: `config/production`.
    ///         - `RUN_MODE=development`: `config/development`.
    /// 2. Environment variables.
    /// 3. CLI command arguments (if any).
    pub fn new() -> Result<Self, SettingsError> {
        let mut cfg = Config::new();

        // We first mixin the configuration that is intended to be shared
        // regardless of `RUN_MODE`.
        cfg.merge(File::with_name("config/default"))?;
        info!("Read config from `config/default`");
        info!("Provided config from `config/default`:\n {:?}", &cfg);

        // Then, we add `RUN_MODE`-determined configuration. Defaults to
        // `development` mode, which takes configuration file at
        // `config/development`.
        if let Ok(run_mode) = env::var("RUN_MODE") {
            match &run_mode[..] {
                "development" => {
                    cfg.merge(
                        File::with_name("config/default").required(false),
                    )?;
                    info!("Reading config from `config/development`");
                    info!(
                        "Provided config from `config/development`:\n {:?}",
                        &cfg
                    );
                }
                "production" => {
                    cfg.merge(
                        File::with_name("config/production").required(false),
                    )?;
                    info!("Reading config from `config/production`");
                    info!(
                        "Provided config from `config/production`:\n {:?}",
                        &cfg
                    );
                }
                other => {
                    warn!("Invalid run mode: \"{}\" given, expected one of \"development\" or \"production\"", other);
                    warn!("Only using configuration from `config/default`!");
                }
            };
        }

        // Then, we mixin environment variables which overrides the keys with a
        // prefix of `APP`.
        //
        // # Example
        //
        // For the key `database.username`, the environment variable
        // `APP_DATABASE_USERNAME` will override the value read from the various
        // configuration files because environment variables have higher
        // precedence.
        cfg.merge(Environment::with_prefix("APP"))?;

        info!("Mixed in configuration from environment variables");
        info!("Final effective configuration provided:\n {:?}", &cfg);

        cfg.try_into()
            .map_err(|e| SettingsError::ConfigError(Box::new(e)))
    }
}

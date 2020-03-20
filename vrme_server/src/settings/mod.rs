//! Settings for the application server.
//!
//! Supports parsing tiered settings format and overriding from environment variables prefixed
//! with `APP_` for settings keys of the same name.
//!
//! Currently support format:
//!
//! - TOML
//! - YAML
//! - JSON

use config::{Config, ConfigError, Environment, File};
use derive_more::Display;
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::env;
use std::net::IpAddr;

/// Errors encountered when trying to determine the settings for the
/// application.
#[derive(Debug, Display)]
pub enum SettingsError {
	/// Failed to find settings file.
	#[display(fmt = "settings file not found at: `{}`", "_0")]
	NotFound(String),
	/// Failed to read settings file due to IO errors.
	#[display(fmt = "failed to read settings due to IO error: `{:?}`", "_0")]
	IOError(std::io::Error),
	/// Failed to parse settings file. Contains illegal syntax.
	#[display(fmt = "invalid syntax: `{}`", "_0")]
	InvalidSyntax(String),
	/// Other settings errors.
	#[display(fmt = "settings error: `{:?}`", "_0")]
	Other(Box<dyn std::error::Error>),
}

impl std::error::Error for SettingsError {}

impl std::convert::From<ConfigError> for SettingsError {
	fn from(error: ConfigError) -> Self {
		match error {
			ConfigError::NotFound(s) => Self::NotFound(s),
			e => Self::Other(Box::new(e)),
		}
	}
}

/// The settings for the application.
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
	pub database: DatabaseSettings,
	pub logging: LoggingSettings,
	pub server: ServerSettings,
}

/// Database settings.
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
	pub username: String,
	pub password: String,
	pub hostname: IpAddr,
	pub port: u16,
	pub database_name: String,
	/// Max number of database connections to maintain in a connection pool.
	#[serde(default = "default_pool_size")]
	pub pool_size: usize,
}

fn default_pool_size() -> usize {
	32
}

/// Logging settings.
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LoggingLevel {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}

/// Server settings.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
	/// Which IP address the application server should bind to.
	pub hostname: IpAddr,
	/// Which port the application server should listen on.
	pub port: u16,
	/// Max JSON payload size in bytes.
	#[serde(default = "default_json_size_limit")]
	pub json_size_limit: usize,
}

fn default_json_size_limit() -> usize {
	4096
}

// For the key `database.username`, the environment variable
// `APP_DATABASE__USERNAME` will override the value read from the various
// configuration files because environment variables have higher
// precedence.
pub const NESTED_SETTINGS_ENV_NAME_SEPARATOR: &str = "__";

impl Settings {
	/// Construct new settings.
	///
	/// # Settings Precedence
	///
	/// By default, the order that settings are overridden is (in order of
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
		debug!("Provided config from `config/default`:\n {:#?}", &cfg);

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
					debug!(
						"Provided config from `config/development`:\n {:#?}",
						&cfg
					);
				}
				"production" => {
					cfg.merge(
						File::with_name("config/production").required(false),
					)?;
					info!("Reading config from `config/production`");
					debug!(
						"Provided config from `config/production`:\n {:#?}",
						&cfg
					);
				}
				other => {
					warn!("Invalid run mode: \"{}\" given, expected one of \"development\" or \
					 	\"production\"", other);
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
		// `APP_DATABASE__USERNAME` will override the value read from the various
		// configuration files because environment variables have higher
		// precedence.
		cfg.merge(
			Environment::with_prefix("APP")
				.separator(NESTED_SETTINGS_ENV_NAME_SEPARATOR),
		)?;

		info!("Mixed in configuration from environment variables");

		match cfg.try_into() {
			Ok(validated_settings) => {
				info!("Settings are validated");
				debug!("Final settings:\n {:#?}", &validated_settings);
				Ok(validated_settings)
			}
			Err(e) => {
				error!("Settings could not be parsed!");
				error!("Error cause:\n {:#?}", &e);
				Err(SettingsError::InvalidSyntax(e.to_string()))
			}
		}
	}
}

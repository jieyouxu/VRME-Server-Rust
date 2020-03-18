//! Logging setup and configuration.

use env_logger;
use log::info;

const DEFAULT_LOGGING_LEVEL: &str = "INFO";

/// Initialize `env_logger`.
///
/// See [sebasmagri/env_logger](https://github.com/sebasmagri/env_logger).
pub fn init() {
	let logging_level = std::env::var("LOG")
		.unwrap_or_else(|_| DEFAULT_LOGGING_LEVEL.to_owned());

	std::env::set_var("LOG", &logging_level);
	std::env::set_var("RUST_LOG", format!("actix_web={}", &logging_level));

	env_logger::Builder::from_env("LOG").init();

	info!("Intialized logger with log level LOG={}", &logging_level);
}

//! Logging setup and configuration.

use env_logger;
use log::info;

const DEFAULT_LOGGING_LEVEL: &str = "INFO";

/// Initialize `env_logger`.
///
/// See [sebasmagri/env_logger](https://github.com/sebasmagri/env_logger).
pub fn init() {
	let logging_level =
		std::env::var("LOG").unwrap_or(DEFAULT_LOGGING_LEVEL.to_owned());

	std::env::set_var("LOG", &logging_level);
	std::env::set_var("RUST_LOG", &logging_level);

	env_logger::init();

	info!("Intialized logger with log level LOG={}", &logging_level);
}

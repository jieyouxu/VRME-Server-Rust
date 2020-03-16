//! Database and connection pool setup and configuration.

use crate::settings::DatabaseSettings;
use deadpool_postgres::config::{Config, ConfigError};
use log::{debug, info};
use tokio_postgres::NoTls;

/// Database connection pool.
///
/// Type alias for `deadpool_postgres::Pool`.
pub type Pool = deadpool_postgres::Pool;

/// Initialize a PostgreSQL database pool.
///
/// # Errors
///
/// Reports the error in `String` description if the construction of a database pool failed.
///
/// See [bikeshedder/deadpool_postgres](https://github.com/bikeshedder/deadpool).
pub fn init_database_pool(
	database_settings: &DatabaseSettings,
) -> Result<Pool, String> {
	let postgre_config = Config {
		user: Some(database_settings.username.clone()),
		password: Some(database_settings.password.clone()),
		dbname: Some(database_settings.database_name.clone()),
		application_name: Some("VRME_Server".to_string()),
		host: Some(database_settings.hostname.to_string()),
		port: Some(database_settings.port),
		..Config::default()
	};

	info!("Attempting to create a PostgreSQL connection pool");
	debug!(
		"Constructing connection pool using settings:\n {:#?}",
		&postgre_config
	);

	postgre_config
		.create_pool(NoTls)
		.map_err(|ConfigError::Message(s)| s)
}

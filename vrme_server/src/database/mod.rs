//! Database and connection pool setup and configuration.

use crate::settings::DatabaseSettings;
use deadpool_postgres::{config, Client, Pool, PoolError};
use derive_more::{Display, From};
use log::{debug, error, info};
use std::convert::From;
use tokio_postgres::NoTls;

/// Database connection pool.
#[derive(From, Clone)]
pub struct ConnectionPool(Pool);

impl ConnectionPool {
	/// Initialize a PostgreSQL database pool from supplied `database_settings`.
	pub fn from_settings(
		database_settings: &DatabaseSettings,
	) -> Result<ConnectionPool, DatabaseError> {
		let database_settings = database_settings.clone();

		let postgres_config = config::Config {
			user: Some(database_settings.username),
			password: Some(database_settings.password),
			host: Some(database_settings.hostname.to_string()),
			port: Some(database_settings.port),
			dbname: Some(database_settings.database_name),
			..config::Config::default()
		};

		info!("Attempting to create a PostgreSQL connection pool");
		debug!(
			"Supplied database settings for initializing connection pool:\n {:?}",
			&postgres_config
		);

		let pool = postgres_config.create_pool(NoTls);

		match pool {
			Ok(pool) => {
				info!("Successfully initialized database connection pool");
				Ok(ConnectionPool(pool))
			}
			Err(e) => {
				error!("Successfully initialized database connection pool");
				Err(e.into())
			}
		}
	}

	/// Get a `deadpool_postgres::Client` to execute queries.
	pub async fn get(&self) -> Result<Client, DatabaseError> {
		self.0.get().await.map_err(|e| e.into())
	}
}

/// Errors related to database.
#[derive(Debug, Display)]
pub enum DatabaseError {
	/// Failed to create a database connection pool.
	#[display(fmt = "failed to create pool: {}", "_0")]
	PoolCreationError(String),
}

impl std::error::Error for DatabaseError {}

impl From<PoolError> for DatabaseError {
	fn from(e: PoolError) -> Self {
		Self::PoolCreationError(e.to_string())
	}
}

impl From<config::ConfigError> for DatabaseError {
	fn from(e: config::ConfigError) -> Self {
		Self::PoolCreationError(e.to_string())
	}
}

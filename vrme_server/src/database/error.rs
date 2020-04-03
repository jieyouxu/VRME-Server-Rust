//! Database errors emitted during initialization, connection establishment and execution.

use actix_web::ResponseError;
use config::ConfigError;
use deadpool_postgres::config::ConfigError as PostgresConfigError;
use deadpool_postgres::PoolError as PostgresPoolError;
use derive_more::Display;
use std::convert::From;

/// Errors related to database.
#[derive(Debug, Display, PartialEq)]
pub enum DatabaseError {
	/// Failed to initalize a database connection pool or to establish connection to the backing
	/// database.
	#[display(fmt = "failed to create connection pool: {}", "_0")]
	PoolCreationError(String),
}

impl From<PostgresPoolError> for DatabaseError {
	fn from(e: PostgresPoolError) -> Self {
		Self::PoolCreationError(e.to_string())
	}
}

impl From<PostgresConfigError> for DatabaseError {
	fn from(e: PostgresConfigError) -> Self {
		Self::PoolCreationError(e.to_string())
	}
}

impl From<ConfigError> for DatabaseError {
	fn from(e: config::ConfigError) -> Self {
		Self::PoolCreationError(e.to_string())
	}
}

impl ResponseError for DatabaseError {}

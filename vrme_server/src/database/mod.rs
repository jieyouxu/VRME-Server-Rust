//! Database and connection pool setup and configuration.

use crate::settings::DatabaseSettings;
use derive_more::{Display, From};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{debug, info};

/// Database connection pool.
#[derive(From, Clone)]
pub struct ConnectionPool(Pool<ConnectionManager<PgConnection>>);

impl ConnectionPool {
	/// Converts the `ConnectionPool` into the inner `r2d2::Pool` type and consumes the wrapper.
	pub fn into_inner(self) -> Pool<ConnectionManager<PgConnection>> {
		self.0
	}
}

/// Errors related to database.
#[derive(Debug, Display)]
pub enum DatabaseError {
	#[display(fmt = "failed to create pool: {}", "_0")]
	PoolCreationError(String),
}

impl std::convert::From<r2d2::Error> for DatabaseError {
	fn from(e: r2d2::Error) -> Self {
		DatabaseError::PoolCreationError(e.to_string())
	}
}

/// Initialize a PostgreSQL database pool.
///
/// # Errors
///
/// Reports the error in `String` description if the construction of a database pool failed.
pub fn create_connection_pool(
	database_settings: &DatabaseSettings,
) -> Result<ConnectionPool, DatabaseError> {
	let database_url = format!(
		"postgres://{user}:{password}@{hostname}:{port}/{database_name}",
		user = database_settings.username,
		password = database_settings.password,
		hostname = database_settings.hostname,
		port = database_settings.port,
		database_name = database_settings.database_name
	);

	info!("Attempting to create a PostgreSQL connection pool");
	debug!("Database URL: {}", &database_url);

	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool = Pool::builder()
		.max_size(database_settings.pool_size as u32)
		.build(manager)?;

	Ok(ConnectionPool(pool))
}

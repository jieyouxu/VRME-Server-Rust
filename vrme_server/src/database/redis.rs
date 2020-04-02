//! In-memory database and connection pool setup and configuration.

use super::error::DatabaseError;
use crate::settings::RedisSettings;

use deadpool_redis::{Config as RedisConfig, Connection, Pool};
use derive_more::From;

/// An connection pool for an in-memory database.
#[derive(From, Clone)]
pub struct InMemoryConnectionPool {
	pool: Pool,
}

impl InMemoryConnectionPool {
	pub fn from_settings(redis_settings: &RedisSettings) -> Result<Self, DatabaseError> {
		let redis_url = build_redis_url(redis_settings);
		let redis_config = RedisConfig {
			url: Some(redis_url),
			..RedisConfig::default()
		};

		let redis_connection_pool = redis_config
			.create_pool()
			.map_err(|e| DatabaseError::PoolCreationError(e.to_string()))?;

		Ok(Self {
			pool: redis_connection_pool,
		})
	}

	pub async fn get(&self) -> Result<Connection, DatabaseError> {
		self.pool.get().await.map_err(|e| e.into())
	}
}

fn build_redis_url(redis_settings: &RedisSettings) -> String {
	// Required redis url format: `redis://[:<passwd>@]<hostname>[:<port>][/<db>]`.
	//
	// See [redis connection parameters](https://docs.rs/redis/0.15.1/redis/#connection-parameters).
	let mut url = String::with_capacity(30);

	// Build protocol name.
	url.push_str("redis://");

	if !redis_settings.password.is_empty() {
		// Build `[:<password>@]`.
		url.push(':');
		url.push_str(&redis_settings.password);
		url.push('@');
	}

	// Build `<hostname>`.
	url.push_str(&redis_settings.hostname.to_string());

	// Build `[:<port>]`.
	url.push(':');
	url.push_str(&redis_settings.port.to_string());

	// Build `[/<db>]`.
	url.push('/');
	url.push_str(&redis_settings.database_number.to_string());

	url
}

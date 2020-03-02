use crate::config::DatabaseConfig;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use log::debug;

/// A database connection pool so we don't have to establish a brand new
/// connection on each request. Uses `diesel::r2d2` for managing pooling.
pub(crate) type DatabaseConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Constructs a database connection URL.
pub(crate) fn construct_database_connection_url(
    database_config: &DatabaseConfig,
) -> String {
    let DatabaseConfig {
        username,
        password,
        netloc,
        port,
        database_name,
    } = database_config;

    let connection_url = format!(
        "postgres://{username}:{password}@{netloc}:{port}/{database_name}",
        username = username,
        password = password,
        netloc = netloc,
        port = port,
        database_name = database_name
    );

    debug!("PostgresSQL url = \"{}\"", connection_url);

    connection_url
}

/// Establish a connection to a PostgreSQL database at the given `database_url`.
pub(crate) fn setup_database_connection_pool(database_url: &str) -> DatabaseConnectionPool {
    debug!(
        "Trying to connect to PostgreSQL database at {}",
        database_url
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool!");

    pool
}

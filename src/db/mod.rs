use crate::config::DatabaseConfig;
use diesel::connection::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use log::{debug, info};

/// A Postgres database connection pool. Connections made to the database can be
/// reused from the pool across multiple requests to avoid the overhead of
/// repeatedly establishing and dropping connections to the database.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// A connection taken from the `DbPool`.
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

const DEFAULT_CONNECTION_POOL_MAX_SIZE: u32 = 15;

/// Setup a connection pool to Postgres so we can reuse connection instead of
/// having to establish a new connection per request.
pub async fn setup_database_connection_pool(
    db_config: &DatabaseConfig,
) -> DbPool {
    debug!(
        "Trying to connect to PostgreSQL database given config: {:?}",
        db_config
    );

    let DatabaseConfig {
        username,
        password,
        netloc,
        port,
        database_name,
    } = db_config;

    let db_url = format!(
        "postgres://{username}:{password}@{netloc}:{port}/{database_name}",
        username = username,
        password = password,
        netloc = netloc,
        port = port,
        database_name = database_name
    );

    debug!("PostgresSQL url = \"{}\"", &db_url);

    PgConnection::establish(&db_url)
        .expect(&format!("Failed to connect to {}", &db_url));

    let manager = ConnectionManager::<PgConnection>::new(&db_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to build connection pool")
}

use crate::config::DatabaseConfig;
use bb8;
use bb8_postgres;
use log::debug;
use tokio_postgres;

/// A Postgres database connection pool. Connections made to the database can be
/// reused from the pool across multiple requests to avoid the overhead of
/// repeatedly establishing and dropping connections to the database.
pub type DatabaseConnectionPool =
    bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>;

const DEFAULT_CONNECTION_POOL_MAX_SIZE: u32 = 15;

/// Setup a connection pool to Postgres so we can reuse connection instead of
/// having to establish a new connection per request.
pub async fn setup_database_connection_pool(
    db_config: &DatabaseConfig,
) -> DatabaseConnectionPool {
    debug!(
        "Trying to connect to PostgreSQL database given config: {:?}",
        db_config
    );

    let mut config = tokio_postgres::Config::new();
    config.user(&db_config.username);
    config.password(&db_config.password);
    config.dbname(&db_config.database_name);
    config.host(&db_config.netloc.to_string());
    config.port(db_config.port);
    config.application_name("VRME-Server");

    let manager = bb8_postgres::PostgresConnectionManager::new(
        config,
        tokio_postgres::NoTls,
    );

    let pool = bb8::Pool::builder()
        .max_size(DEFAULT_CONNECTION_POOL_MAX_SIZE)
        .build(manager)
        .await
        .expect("failed to construct connection pool to postgres");

    pool
}

pub mod accounts;
pub mod auth;
pub mod database;
mod json_error_handler;
pub mod logging;
pub mod service_errors;
pub mod settings;
pub mod types;
mod welcome;

use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::web;
use actix_web::HttpServer;
use actix_web::{middleware, App};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::{error, info};
use std::net;

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main entry point to the Virtual Reality Meeting Environment backend server.
///
/// # Panics
///
/// - If settings provided are invalid, the server instance will panic with
///   error messages to indicate erroneous configuration.
/// - Panics if failed to create a database connection pool.
///
/// # Additional References
///
/// Built with [actix/actix-web](https://github.com/actix/actix-web).
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	logging::init();
	welcome::welcome()?;

	info!("VRME_Server version {}", VERSION);

	let settings = read_settings();

	let socket_address =
		net::SocketAddr::new(settings.server.hostname, settings.server.port);

	info!("Server listening on http://{}", &socket_address);

	let connection_pool = create_connection_pool(&settings.database);

	HttpServer::new(move || {
		let _auth_middleware =
			HttpAuthentication::bearer(auth::middleware::identity_validator);

		let rate_limit_memory_store = MemoryStore::new();

		App::new()
			.wrap(
				middleware::DefaultHeaders::new().header("X-Version", VERSION),
			)
			.wrap(middleware::Compress::default())
			.wrap(
				// Rate limiting
				RateLimiter::new(
					MemoryStoreActor::from(rate_limit_memory_store.clone())
						.start(),
				)
				.with_interval(std::time::Duration::from_secs(60))
				.with_max_requests(100),
			)
			.wrap(middleware::Logger::default())
			.data(settings.clone())
			.app_data(
				web::JsonConfig::default()
					.limit(settings.server.json_size_limit)
					.error_handler(json_error_handler::handle_json_error),
			)
			.data(connection_pool.clone())
			.route(
				"/register",
				web::post().to(accounts::register::handle_registration),
			)
			.route("/login", web::post().to(auth::login::handle_login))
			.route(
				"/accounts/uuid",
				web::get().to(accounts::get_uuid::handle_get_uuid),
			)
	})
	.bind(socket_address)?
	.run()
	.await
}

fn read_settings() -> settings::Settings {
	match settings::Settings::new() {
		Ok(s) => s,
		Err(e) => {
			error!("Invalid config provided:\n {:?}", &e);
			panic!("Invalid config provided:\n {:?}", &e);
		}
	}
}

fn create_connection_pool(
	settings: &settings::DatabaseSettings,
) -> database::ConnectionPool {
	match database::ConnectionPool::from_settings(settings) {
		Ok(pool) => pool,
		Err(e) => {
			error!("Failed to create connection pool: {:?}", &e);
			panic!("Failed to create connection pool: {:?}", &e);
		}
	}
}

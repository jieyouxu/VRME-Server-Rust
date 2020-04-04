pub mod accounts;
pub mod auth;
pub mod avatars;
pub mod database;
mod json_error_handler;
pub mod logging;
pub mod meetings;
pub mod presentations;
pub mod service_errors;
pub mod settings;
pub mod types;
mod welcome;

use crate::database::postgresql::PersistentConnectionPool;
use crate::settings::Settings;

use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::web;
use actix_web::HttpServer;
use actix_web::{middleware, App};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::{error, info};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig as TlsConfig};
use std::fs::File;
use std::io::BufReader;
use std::net;

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main entry point to the Virtual Reality Meeting Environment backend server.
///
/// # Panics
///
/// - If settings provided are invalid, the server instance will panic with error messages to
///   indicate erroneous configuration.
/// - Panics if failed to create a database connection pool.
///
/// # Additional References
///
/// Built with [actix/actix-web](https://github.com/actix/actix-web).
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	color_backtrace::install();
	human_panic::setup_panic!();

	logging::init();
	welcome::welcome()?;

	info!("VRME_Server version {}", VERSION);

	let settings = read_settings();

	let socket_address =
		net::SocketAddr::new(settings.server.hostname.clone(), settings.server.port);
	info!("Server listening on http://{}", &socket_address);

	let persistent_connection_pool = create_persistent_connection_pool(&settings.database);

	// Curried closure: required data `settings` and `connection_pool` needs to be passed in by
	// value (by cloning) to prevent moving values.
	//
	// In pseduo-Haskell type signature: `create_app :: (Settings, ConnectionPool) -> move () -> App`.
	let create_app =
		|settings: Settings, persistent_connection_pool: PersistentConnectionPool| {
			move || {
				let auth_middleware =
					HttpAuthentication::bearer(auth::middleware::identity_validator);
				let rate_limit_memory_store = MemoryStore::new();

				App::new()
					.wrap(middleware::DefaultHeaders::new().header("X-Version", VERSION))
					.wrap(middleware::Compress::default())
					.wrap(
						// Rate limiting
						RateLimiter::new(
							MemoryStoreActor::from(rate_limit_memory_store.clone()).start(),
						)
						.with_interval(std::time::Duration::from_secs(
							settings.rate_limiting.cooldown_duration,
						))
						.with_max_requests(settings.rate_limiting.max_requests),
					)
					.wrap(middleware::Logger::default())
					.data(settings.clone())
					.app_data(
						web::JsonConfig::default()
							.limit(settings.server.json_size_limit)
							.error_handler(json_error_handler::handle_json_error),
					)
					.data(persistent_connection_pool.clone())
					.route(
						"/register",
						web::post().to(accounts::register::handle_registration),
					)
					.service(
						web::resource("/account")
							.wrap(auth_middleware.clone())
							.route(web::delete().to(accounts::delete::handle_delete_account)),
					)
					.route("/login", web::post().to(auth::login::handle_login))
					.service(
						web::resource("/logout")
							.wrap(auth_middleware.clone())
							.route(web::post().to(auth::logout::handle_logout)),
					)
					.route(
						"/accounts/uuid",
						web::get().to(accounts::get_uuid::handle_get_uuid),
					)
					.service(
						web::scope("/accounts/{uuid}")
							.service(web::resource("").wrap(auth_middleware.clone()).route(
								web::put().to(accounts::update_info::handle_update_user_account),
							))
							.service(
								web::resource("/avatar")
									.route(web::get().to(avatars::get_avatar::handle_get_avatar)),
							)
							.service(
								web::resource("/avatar")
									.wrap(auth_middleware.clone())
									.route(web::post().to(avatars::upload::handle_upload_avatar)),
							)
							.service(
								web::resource("/avatar")
									.wrap(auth_middleware.clone())
									.route(
										web::delete()
											.to(avatars::delete_avatar::handle_delete_avatar),
									),
							),
					)
					.service(
						web::scope("/meetings").service(
							web::resource("")
								.wrap(auth_middleware.clone())
								.route(web::post().to(meetings::init_session::handle_init_session)),
						),
					)
					.service(
						web::scope("/meetings/{meeting_id}")
							.wrap(auth_middleware.clone())
							.service(web::resource("").route(
								web::get().to(
									meetings::get_session_info::handle_get_meeting_session_info,
								),
							))
							.service(
								web::resource("/listener").route(
									web::post().to(meetings::add_listener::handle_add_listener),
								),
							)
							.service(web::resource("/leave").route(
								web::post().to(meetings::leave::handle_leave_meeting_session),
							))
							.service(
								web::resource("/presentation")
									.route(web::post().to(
										presentations::upload::handle_upload_presentation_slides,
									))
									.route(web::get().to(
										presentations::get_presentation::handle_get_presentation,
									))
									.route(
										web::delete()
											.to(presentations::delete::handle_delete_presentation),
									),
							),
					)
			}
		};

	let server = HttpServer::new(create_app(
		settings.clone(),
		persistent_connection_pool.clone(),
	))
	.bind(socket_address)?;

	match &settings.tls {
		Some(tls_settings) if tls_settings.use_tls => {
			// Load SSL keys
			let mut tls_config = TlsConfig::new(NoClientAuth::new());
			let cert_file = &mut BufReader::new(
				File::open(&tls_settings.cert_path).expect("`cert.pem` not found"),
			);
			let key_file = &mut BufReader::new(
				File::open(&tls_settings.key_path).expect("`key.pem` not found"),
			);
			let cert_chain = certs(cert_file).unwrap();
			let mut keys = rsa_private_keys(key_file).unwrap();
			tls_config
				.set_single_cert(cert_chain, keys.remove(0))
				.unwrap();

			let tls_socket_address =
				net::SocketAddr::new(settings.server.hostname.clone(), tls_settings.port);

			info!(
				"Server (TLS) listening on https://localhost:{}",
				&tls_socket_address
			);

			server.bind_rustls(socket_address, tls_config)?.run().await
		}
		_ => {
			info!("Server not using TLS");
			server.run().await
		}
	}
}

#[inline]
fn read_settings() -> settings::Settings {
	match settings::Settings::new() {
		Ok(s) => s,
		Err(e) => {
			error!("Invalid config provided:\n {:?}", &e);
			panic!("Invalid config provided:\n {:?}", &e);
		}
	}
}

#[inline]
fn create_persistent_connection_pool(
	settings: &settings::DatabaseSettings,
) -> PersistentConnectionPool {
	match PersistentConnectionPool::from_settings(settings) {
		Ok(pool) => pool,
		Err(e) => {
			error!("Failed to create postgresql connection pool: {:?}", &e);
			panic!("Failed to create postgresql connection pool: {:?}", &e);
		}
	}
}

pub mod accounts;
pub mod database;
pub mod logging;
pub mod service_errors;
pub mod settings;

use actix_web::error::{Error, JsonPayloadError};
use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use log::{error, info};
use service_errors::ServiceError;
use std::io::Write;
use std::net;

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Max JSON size in kB
const MAX_JSON_SIZE: usize = 32;

/// Main entry point to the Virtual Reality Meeting Environment backend server.
///
/// # Panics
/// If configuration provided is invalid, the server instance will panic with
/// error messages to indicate erroneous configuration.
///
/// # Errors
///
/// Returns `std::io::Error` if the server instance fails to bind to the
/// provided socket address.
///
/// # Additional References
///
/// Built with [actix/actix-web](https://github.com/actix/actix-web).
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	logging::init();

	print_welcome_info()?;
	info!("VRME_Server version {}", VERSION);

	let settings = settings::Settings::new().unwrap_or_else(|ref e| {
		error!("Failed to load settings:\n {:#?}", e);
		std::process::exit(1);
	});

	let socket_addr =
		net::SocketAddr::new(settings.server.hostname, settings.server.port);

	info!("Server listening on http://{}", &socket_addr);

	let connection_pool = database::init_database_pool(&settings.database)
		.unwrap_or_else(|ref e| {
			error!("Failed to initialize PostgreSQL connection pool");
			error!("Error cause: {}", e);
			std::process::exit(1);
		});

	HttpServer::new(move || {
		App::new()
			.wrap(middleware::Logger::default())
			.app_data(
				web::JsonConfig::default()
					.limit(MAX_JSON_SIZE)
					.error_handler(handle_json_error),
			)
			.app_data(web::Data::new(settings.clone()))
			.data(connection_pool.clone())
			.service(
				web::resource("/register").route(
					web::post().to(accounts::register::handle_registration),
				),
			)
	})
	.bind(socket_addr)?
	.run()
	.await
}

const NAME: &[u8] = br#"
 __      __ _____   __  __  ______    _____  ______  _____ __      __ ______  _____
 \ \    / /|  __ \ |  \/  ||  ____|  / ____||  ____||  __ \\ \    / /|  ____||  __ \
  \ \  / / | |__) || \  / || |__    | (___  | |__   | |__) |\ \  / / | |__   | |__) |
   \ \/ /  |  _  / | |\/| ||  __|    \___ \ |  __|  |  _  /  \ \/ /  |  __|  |  _  /
    \  /   | | \ \ | |  | || |____   ____) || |____ | | \ \   \  /   | |____ | | \ \
     \/    |_|  \_\|_|  |_||______| |_____/ |______||_|  \_\   \/    |______||_|  \_\
                                ______
                               |______|
"#;

fn print_welcome_info() -> std::io::Result<()> {
	let stdout = std::io::stdout();
	let mut handle = stdout.lock();
	handle.write_all(NAME)?;
	Ok(())
}

fn handle_json_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
	let err_msg = match err {
		JsonPayloadError::Overflow => {
			&format!("Payload size exceeds the max limit: {} kB", MAX_JSON_SIZE)
		}
		JsonPayloadError::ContentType => {
			"Invalid `Content-Type` header: use `application/json`"
		}
		_ => "Invalid JSON payload",
	};

	ServiceError::BadRequest(format!(
		"Failed to parse payload as JSON: {}",
		err_msg
	))
	.into()
}

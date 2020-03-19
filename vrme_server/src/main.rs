pub mod accounts;
pub mod database;
pub mod logging;
pub mod service_errors;
pub mod settings;
pub(crate) mod welcome;

use actix_web::error::{Error, JsonPayloadError};
use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use log::{error, info};
use serde_json;
use service_errors::ServiceError;
use std::net;

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Max JSON size in kB.
const MAX_JSON_SIZE: usize = 4096;

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

	welcome::welcome()?;

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
			.data(settings.clone())
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

/// Custom JSON error handler.
///
/// When a JSON error is encountered, it returns a detailed error message for the malformed payload,
/// both for syntactical malformed payload and for semantically malformed payload such as missing
/// fields.
///
/// # Example Error Response
///
/// ```http
/// HTTP/1.1 400 Bad Request
/// Content-Type: application/json
///
/// {
///     "cause": "bad-request",
///     "message": "Invalid JSON at [line = 1, col = 1]"
/// }
/// ```
fn handle_json_error(err: JsonPayloadError, _req: &HttpRequest) -> Error {
	let err_msg = match err {
		JsonPayloadError::Overflow => {
			format!("Payload size exceeds the max limit: {} kB", MAX_JSON_SIZE)
		}
		JsonPayloadError::ContentType => {
			"Invalid `Content-Type` header: use `application/json`".to_string()
		}
		JsonPayloadError::Deserialize(ref e) => {
			use serde_json::error::Category;
			match e.classify() {
				Category::Syntax => format!(
					"Invalid JSON at [line = {}, col = {}]",
					e.line(),
					e.column()
				),
				Category::Data => {
					// Unfortunately `serde_json`'s Errors are opaque and do not contain useful
					// information such as missing fields.
					//
					// Hence, we can only exploit it's `std::fmt::Display`'s implementation to get
					// which field is missing that is required.
					format!(
                        "Missing required field(s) and/or values have invalid types: {}",
                        e.to_string()
                    )
				}
				Category::Eof => {
					"Expected EOF when trying to parse JSON".to_string()
				}
				Category::Io => "IO error when parsing JSON".to_string(),
			}
		}
		_ => "Invalid JSON payload".to_string(),
	};

	ServiceError::BadRequest(format!(
		"Failed to parse payload as JSON: {}",
		err_msg
	))
	.into()
}

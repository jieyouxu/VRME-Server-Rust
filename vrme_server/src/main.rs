/// Logging support.
pub mod logging;
/// Supports tiered settings, from settings files and environment variables.
pub mod settings;

use actix_web::{web, App, HttpServer};
use log::{error, info};
use std::io::Write;
use std::net;

/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main entry point to the Virtual Reality Meeting Environment backend server.
///
/// # Panics
///
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
/// Build with [actix/actix-web](https://github.com/actix/actix-web).
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	logging::init();

	print_welcome_info()?;
	info!("VRME_Server version {}", VERSION);

	let settings = settings::Settings::new().unwrap_or_else(|e| {
		error!("Failed to load settings:\n {:#?}", &e);
		std::process::exit(1);
	});

	let socket_addr =
		net::SocketAddr::new(settings.server.hostname, settings.server.port);

	info!("Server listening on http://{}", &socket_addr);

	HttpServer::new(move || {
		App::new().app_data(web::Data::new(settings.clone()))
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

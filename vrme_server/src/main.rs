/// Logging support.
pub mod logging;
/// Supports tiered settings, from settings files and environment variables.
pub mod settings;

use actix_web::{App, HttpServer};
use log::{error, info};
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

    info!("VRME_Server version {}", VERSION);

    let settings = settings::Settings::new().unwrap_or_else(|e| {
        error!("Failed to load settings:\n {:#?}", &e);
        std::process::exit(1);
    });

    let socket_addr =
        net::SocketAddr::new(settings.server.hostname, settings.server.port);

    info!("Server listening on http://{}", &socket_addr);

    HttpServer::new(|| App::new())
        .bind(socket_addr)?
        .run()
        .await
}

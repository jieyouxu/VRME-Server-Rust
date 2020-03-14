pub mod config;

use actix_web::{get, web, App, HttpServer};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new()
        .bind()
        .await
}

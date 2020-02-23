use crate::config;
use actix_web::{get, App, HttpServer, Responder};
use log::info;
use std::net;

#[actix_rt::main]
pub(crate) async fn start_server(config: &'static config::Config) -> std::io::Result<()> {
    info!(
        "starting server at {}:{}",
        config.server.address, config.server.port
    );
    println!(
        "starting server at {}:{}",
        config.server.address, config.server.port
    );
    HttpServer::new(|| App::new().service(home))
        .bind(make_socket_addr(config))?
        .run()
        .await
}

fn make_socket_addr(config: &config::Config) -> net::SocketAddr {
    let (ip_addr, port) = (config.server.address, config.server.port);
    net::SocketAddr::new(ip_addr, port)
}

#[get("/")]
pub(crate) async fn home() -> impl Responder {
    "server is running...".to_string()
}

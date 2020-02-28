use crate::config;
use actix_web::middleware::Logger;
use actix_web::{error, web, App, HttpResponse, HttpServer};
use log::info;
use serde::{Deserialize, Serialize};
use std::net;

use crate::account::register;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MalformedJsonResponse {
    message: String,
}

impl Default for MalformedJsonResponse {
    fn default() -> MalformedJsonResponse {
        Self {
            message: "malformed JSON".to_owned(),
        }
    }
}

#[actix_rt::main]
pub(crate) async fn start(
    config: &'static config::Config,
) -> std::io::Result<()> {
    info!(
        "Starting server at {}:{}",
        config.server.address, config.server.port
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest()
                        .json(MalformedJsonResponse::default()),
                )
                .into()
            }))
            .route("/register", web::post().to(register::handle_register_user))
    })
    .bind(make_socket_addr(config))?
    .run()
    .await
}

fn make_socket_addr(config: &config::Config) -> net::SocketAddr {
    let (ip_addr, port) = (config.server.address, config.server.port);
    net::SocketAddr::new(ip_addr, port)
}

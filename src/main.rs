use lazy_static::lazy_static;
use log::{debug, error, info};
use pretty_env_logger;
use std::env;

mod config;
mod server;

const DEFAULT_LOG_LEVEL: &str = "warn";

lazy_static! {
    static ref CONFIG: config::Config = match config::get_config("config.toml") {
        Ok(c) => c,
        Err(e) => {
            error!("failed to read config");
            error!("error cause: {:#?}", e);
            panic!("failed to read config");
        }
    };
}

fn main() {
    setup_log_level_env();
    pretty_env_logger::init();

    debug!(
        "executable current directory = {:?}",
        env::current_dir().unwrap()
    );

    debug!("config given = {:#?}", &*CONFIG);

    server::start_server(&CONFIG).unwrap();
}

fn setup_log_level_env() {
    let log_level = env::vars()
        .find(|(key, _)| key.eq_ignore_ascii_case("LOG_LEVEL"))
        .map(|(_, val)| val)
        .unwrap_or_else(|| DEFAULT_LOG_LEVEL.to_owned());
    env::set_var("LOG_LEVEL", &log_level);
    info!("using LOG_LEVEL = {}", &log_level);
}

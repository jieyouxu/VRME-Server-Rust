use log::{debug, info, warn};
use pretty_env_logger;
use std::env;

mod config;

const DEFAULT_LOG_LEVEL: &'static str = "warn";

fn main() {
    setup_log_level_env();
    pretty_env_logger::init();

    let cfg = match config::get_config("../../config.toml") {
        Ok(c) => c,
        Err(e) => {
            warn!("failed to read config");
            debug!("error cause: {:#?}", e);
            return;
        }
    };

    debug!("config given is:\n {:#?}", &cfg);

    todo!()
}

fn setup_log_level_env() {
    let mut log_level = String::from(DEFAULT_LOG_LEVEL);

    for (env_key, env_value) in env::vars() {
        if env_key.eq_ignore_ascii_case("LOG_LEVEL") {
            log_level = env_value;
        }
    }

    env::set_var("LOG_LEVEL", &log_level);
    info!("using LOG_LEVEL = {}", &log_level);
}

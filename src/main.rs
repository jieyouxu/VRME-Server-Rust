#![forbid(unsafe_code)]

use lazy_static::lazy_static;
use log::{debug, error};

mod account;
mod config;
mod db;
mod logger;
mod server;
mod types;

lazy_static! {
    /// Configuration for the VRME server.
    static ref CONFIG: config::Config = {
        let path = config_path();
        config::get_config(&path).unwrap_or_else(|e| {
            error!("Failed to read config");
            error!("Error cause: {:#?}", e);
            panic!("Failed to read config");
        })
    };
}

/// Default path to try to look for the configuration file.
const DEFAULT_CONFIG_PATH: &str = "config.toml";

/// Obtain the config file path either from the environment variable
/// `CONFIG_PATH`, or defaults to `DEFAULT_CONFIG_PATH`.
fn config_path() -> String {
    std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string())
}

/// Entry point to start the server.
pub fn main() {
    logger::init();

    debug!(
        "Executable current directory = {:?}",
        std::env::current_dir().unwrap()
    );

    debug!("Config given = {:#?}", &*CONFIG);

    server::start(&CONFIG).unwrap();
}

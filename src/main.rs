use lazy_static::lazy_static;
use log::{debug, error};

mod config;
mod logger;
mod server;

lazy_static! {
    static ref CONFIG: config::Config = match config::get_config("config.toml") {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read config");
            error!("Error cause: {:#?}", e);
            panic!("Failed to read config");
        }
    };
}

fn main() {
    logger::init();

    debug!(
        "Executable current directory = {:?}",
        std::env::current_dir().unwrap()
    );

    debug!("Config given = {:#?}", &*CONFIG);

    server::start(&CONFIG).unwrap();
}

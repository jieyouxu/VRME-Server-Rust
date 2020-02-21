use log::debug;
use pretty_env_logger;

mod config;

fn main() {
    pretty_env_logger::init();

    let cfg = match config::get_config("../../config.toml") {
        Ok(c) => c,
        Err(e) => {
            debug!("failed to read config");
            debug!("error cause: {:#?}", e);
            return;
        }
    };

    debug!("config given is:\n {:#?}", &cfg);

    println!("Hello, world!");
}

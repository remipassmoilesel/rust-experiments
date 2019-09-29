extern crate log;
extern crate proxy;
extern crate simple_logger;

use log::{error, info, Level};

use proxy::start_proxy;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let config_path = String::from("proxy-config.yml");
    let result = start_proxy(config_path);
    match result {
        Err(e) => error!("Proxy start failed: {:?}", e),
        _ => info!("Proxy stopped"),
    }
}

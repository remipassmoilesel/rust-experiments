
extern crate proxy;

use proxy::start_proxy;

fn main() {
    let config_path = String::from("config.yml");
    start_proxy(config_path)
}

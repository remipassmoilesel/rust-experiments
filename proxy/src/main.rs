extern crate proxy;

use proxy::start_proxy;

fn main() {
    let config_path = String::from("proxy-config.yml");
    let result = start_proxy(config_path);
    match result {
        Err(e) => eprintln!("Proxy start failed: {:?}", e),
        _ => println!("Proxy stopped"),
    }
}

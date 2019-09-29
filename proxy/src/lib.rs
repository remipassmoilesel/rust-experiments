
mod configuration;
use crate::configuration::Configuration;

pub fn start_proxy(config_path: String){
    let config = Configuration::new(config_path);
    println!("{:?}", config)
}

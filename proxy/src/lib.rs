mod configuration;
use crate::configuration::Configuration;
use std::error::Error;

pub fn start_proxy(config_path: String) -> Result<(), Box<dyn Error>> {
    let config = Configuration::new(config_path)?;
    for proxy_section in config.proxy_sections {
        println!("Proxy configuration block: {:#?}", proxy_section)
    }
    Ok(())
}

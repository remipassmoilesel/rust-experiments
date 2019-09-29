mod authentication_filter;
mod configuration;
mod proxy;
mod server;
use crate::configuration::Configuration;

use crate::server::Server;
use log::info;
use std::error::Error;

extern crate log;

pub fn start_proxy(config_path: String) -> Result<(), Box<dyn Error>> {
    let config = Configuration::new(config_path)?;
    let server = Server::new(config.clone());

    display_config_banner(config);
    server.start()
}

fn display_config_banner(config: Configuration) {
    let proxy_section_names: Vec<String> = config
        .proxy_sections
        .iter()
        .map(|c| c.name.as_ref().unwrap_or(&String::from("Unnamed")).clone())
        .collect();
    info!(
        "Proxy server starting on: {:?}",
        config.server_section.connection_string,
    );
    info!(
        "With proxy configuration sections: {}",
        proxy_section_names.join(", ")
    );
}

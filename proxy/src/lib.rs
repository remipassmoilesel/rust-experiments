extern crate log;

use std::error::Error;

use log::info;

use crate::configuration::Configuration;
use crate::server::Server;

mod authentication;
mod config_resolver;
mod configuration;
mod proxy;
mod server;

pub fn start_proxy(config_path: String) -> Result<(), Box<dyn Error>> {
    let config = Configuration::new(config_path)?;
    let server = Server::new(config.clone());

    display_config_banner(config);
    server.start()
}

fn display_config_banner(config: Configuration) {
    info!(
        "Proxy server starting on: {:?}",
        config.server_section.connection_string,
    );
    info!("Server configuration: {:?}", config.server_section,);
    info!("Proxy configuration sections: ");
    config
        .proxy_sections
        .iter()
        .for_each(|sec| info!("{:?}", sec));
}

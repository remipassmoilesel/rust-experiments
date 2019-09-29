mod configuration;
mod proxy;
mod server;
use crate::configuration::Configuration;
use crate::proxy::Proxy;
use crate::server::Server;
use std::error::Error;
use std::rc::Rc;

pub fn start_proxy(config_path: String) -> Result<(), Box<dyn Error>> {
    let config = Configuration::new(config_path)?;
    let proxy = Proxy::new(config.clone());
    let server = Server::new(config.clone(), proxy);

    display_config_banner(config);
    server.start()
}

fn display_config_banner(config: Configuration) {
    let proxy_section_names: Vec<String> = config
        .proxy_sections
        .iter()
        .map(|c| c.name.as_ref().unwrap_or(&String::from("Unnamed")).clone())
        .collect();
    println!(
        "\nProxy server starting on: {:?}",
        config.server_section.connection_string,
    );
    println!(
        "With proxy configuration sections: {}\n",
        proxy_section_names.join(", ")
    );
}

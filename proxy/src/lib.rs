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
    let server = Server::new(proxy);

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
        "\nProxy server starting with proxy configurations: {}",
        proxy_section_names.join(", ")
    );
    println!(
        "On: {:?} {:?}",
        config.server_section.hosts,
        config.server_section.port,
    );
}

extern crate yaml_rust;

use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;

use yaml_rust::YamlLoader;

use self::yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub server_section: ServerSection,
    pub proxy_sections: Vec<ProxySection>,
}

impl Configuration {
    pub fn new(path: String) -> Result<Configuration, ConfigurationLoadError> {
        let file_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                return Err(ConfigurationLoadError {
                    message: format!("Cannot read file: {:?} {}", e.kind(), &path),
                });
            }
        };

        let all_documents = match YamlLoader::load_from_str(&file_content) {
            Ok(docs) => docs,
            _ => {
                return Err(ConfigurationLoadError {
                    message: format!("Invalid file: {}, try yamllint to spot errors", &path),
                });
            }
        };

        let server_section: Vec<ServerSection> = all_documents
            .iter()
            .filter(|doc| !doc["server"].is_badvalue())
            .take(1)
            .map(|doc| ServerSection::new(&doc["server"]))
            .collect();

        let server_section = match server_section.get(0) {
            Some(x) => x.clone(),
            None => {
                return Err(ConfigurationLoadError {
                    message: String::from("Server configuration section is mandatory"),
                })
            }
        };

        let proxy_sections: Vec<ProxySection> = all_documents
            .iter()
            .filter(|doc| doc["proxy"].is_array())
            .flat_map(|doc| doc["proxy"].as_vec().unwrap().iter())
            .map(|doc| ProxySection::new(doc))
            .collect();

        Ok(Configuration {
            server_section,
            proxy_sections,
        })
    }
}

// TODO: some parameters must be mandatory
#[derive(Debug, Clone)]
pub struct ProxySection {
    pub name: Option<String>,
    pub path: Option<String>,
    pub forward: Option<String>,
    pub secret: Option<String>,
    pub allowed_origins: Option<String>,
}

impl ProxySection {
    fn new(yaml: &Yaml) -> ProxySection {
        let name = yaml_to_string_option("name", yaml);
        let path = yaml_to_string_option("matching_path", yaml);
        let forward = yaml_to_string_option("forward_to", yaml);
        let secret = yaml_to_string_option("secret", yaml);
        let allowed_origins = yaml_to_string_option("allowed_origins", yaml);

        ProxySection {
            name,
            path,
            forward,
            secret,
            allowed_origins,
        }
    }
}

// TODO: some parameters must be mandatory
#[derive(Debug, Clone)]
pub struct ServerSection {
    pub connection_string: String,
    pub authorization_header: String,
}

impl ServerSection {
    fn new(yaml: &Yaml) -> ServerSection {
        let connection_string: Option<String> =
            yaml["connection_string"].as_str().map(|s| String::from(s));
        let authorization_header: Option<String> = yaml["authorization_header"]
            .as_str()
            .map(|s| String::from(s));
        ServerSection {
            connection_string: connection_string.unwrap_or(String::from("127.0.0.1:3000")),
            authorization_header: authorization_header.unwrap_or(String::from("Authorization")),
        }
    }
}

fn yaml_to_string_option(name: &str, yaml: &Yaml) -> Option<String> {
    match &yaml[name] {
        Yaml::String(x) => Some(x.clone()),
        _ => None,
    }
}

#[derive(Debug)]
pub struct ConfigurationLoadError {
    message: String,
}

impl Error for ConfigurationLoadError {}

impl Display for ConfigurationLoadError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

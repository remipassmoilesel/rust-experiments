extern crate yaml_rust;

use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::sync::Arc;

use hyper::Uri;
use log::error;
use log::info;
use regex::Regex;
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
                });
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

    pub fn section_from_uri(&self, uri: &Uri) -> Option<ProxySection> {
        let url = format!("{}{}", &uri.host().unwrap_or_else(|| ""), &uri.path());
        let matching: Vec<&ProxySection> = self
            .proxy_sections
            .iter()
            .filter(|sec| sec.matching_path_regex.is_match(&url))
            .collect();

        matching.get(0).map(|refr| (**refr).clone())
    }
}

// TODO: some parameters must be mandatory
#[derive(Debug, Clone)]
pub struct ProxySection {
    pub name: Option<String>,
    pub matching_path: String,
    pub matching_path_regex: Regex,
    pub forward_to: String,
    pub secret: Option<String>,
    pub allowed_origins: Vec<String>,
}

impl ProxySection {
    fn new(yaml: &Yaml) -> ProxySection {
        let name = yaml_to_string_option("name", yaml);
        let matching_path = yaml_to_string_option("matching_path", yaml).unwrap();
        let matching_path_regex = Regex::new(&matching_path).unwrap();
        let forward_to = yaml_to_string_option("forward_to", yaml).unwrap();
        let secret = yaml_to_string_option("secret", yaml);

        let allowed_origins_str = yaml_to_string_option("allowed_origins", yaml).unwrap_or(String::from(""));
        let allowed_origins: Vec<String> = allowed_origins_str
            .split(",")
            .filter(|s| s.len() > 0)
            .map(|s| String::from(s))
            .collect();

        ProxySection {
            name,
            matching_path,
            matching_path_regex,
            forward_to,
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
        let connection_string: Option<String> = yaml["connection_string"].as_str().map(|s| String::from(s));
        let authorization_header: Option<String> = yaml["authorization_header"].as_str().map(|s| String::from(s));
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use log::{error, info, Level};
    use regex::Regex;

    use crate::configuration::ServerSection;

    use super::*;

    extern crate log;
    extern crate simple_logger;

    #[test]
    fn should_match_configuration_2() {
        setup();
        let config = test_config();
        let section = config.section_from_uri(&Uri::from_str("http://localhost:9990/path-2").unwrap());
        assert_eq!(section.is_some(), true);
        assert_eq!(section.unwrap().name, Some(String::from("section-2")));
    }

    #[test]
    fn should_not_match_configuration() {
        let config = test_config();
        let section = config.section_from_uri(&Uri::from_str("nothing-should-match").unwrap());
        assert_eq!(section.is_none(), true);
    }

    fn setup() {
        simple_logger::init();
    }

    fn test_config() -> Configuration {
        Configuration {
            server_section: ServerSection {
                connection_string: String::from("127.0.0.1:8787"),
                authorization_header: String::from("Authorization"),
            },
            proxy_sections: vec![
                ProxySection {
                    name: Some(String::from("section-1")),
                    matching_path: String::from("/path-1"),
                    matching_path_regex: Regex::new(&"/path-1").unwrap(),
                    forward_to: String::from("http://localhost:9990"),
                    secret: None,
                    allowed_origins: vec![],
                },
                ProxySection {
                    name: Some(String::from("section-2")),
                    matching_path: String::from("/path-2"),
                    matching_path_regex: Regex::new(&"/path-2").unwrap(),
                    forward_to: String::from("http://localhost:9990"),
                    secret: None,
                    allowed_origins: vec![],
                },
            ],
        }
    }
}

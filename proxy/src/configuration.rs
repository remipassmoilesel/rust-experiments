extern crate yaml_rust;

use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;

use yaml_rust::YamlLoader;

use self::yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Configuration {
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

        let sections: Vec<ProxySection> = all_documents
            .iter()
            .filter(|doc| doc["proxy"].is_array())
            .flat_map(|doc| doc["proxy"].as_vec().unwrap().iter())
            .map(|doc| ProxySection::new(doc))
            .collect();

        Ok(Configuration {
            proxy_sections: sections,
        })
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
        let path = yaml_to_string_option("path", yaml);
        let forward = yaml_to_string_option("forward", yaml);
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

fn yaml_to_string_option(name: &str, yaml: &Yaml) -> Option<String> {
    match &yaml[name] {
        Yaml::String(x) => Some(x.clone()),
        _ => None,
    }
}

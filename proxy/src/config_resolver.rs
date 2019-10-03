use hyper::Uri;
use log::error;
use log::info;

use crate::configuration::{Configuration, ProxySection};
use std::sync::Arc;

pub struct ProxyConfigResolver {
    configuration: Arc<Configuration>,
}

impl ProxyConfigResolver {
    pub fn new(configuration: Arc<Configuration>) -> ProxyConfigResolver {
        ProxyConfigResolver { configuration }
    }

    pub fn section_from_uri(&self, uri: &Uri) -> Option<ProxySection> {
        let url = format!("{}{}", &uri.host().unwrap_or_else(|| ""), &uri.path());
        let matching: Vec<&ProxySection> = self
            .configuration
            .proxy_sections
            .iter()
            .filter(|sec| sec.matching_path_regex.is_match(&url))
            .collect();

        matching.get(0).map(|refr| (**refr).clone())
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
        let resolver = ProxyConfigResolver::new(Arc::new(config));

        let section = resolver.section_from_uri(&Uri::from_str("http://localhost:9990/path-2").unwrap());
        assert_eq!(section.is_some(), true);
        assert_eq!(section.unwrap().name, Some(String::from("section-2")));
    }

    #[test]
    fn should_not_match_configuration() {
        let config = test_config();
        let resolver = ProxyConfigResolver::new(Arc::new(config));

        let section = resolver.section_from_uri(&Uri::from_str("nothing-should-match").unwrap());
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

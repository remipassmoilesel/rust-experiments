use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request};

use crate::configuration::{Configuration, ProxySection};
use crate::proxy::ProxyError;

pub struct AuthenticationFilter {
    configuration: Arc<Configuration>,
}

impl AuthenticationFilter {
    pub fn new(configuration: Arc<Configuration>) -> AuthenticationFilter {
        AuthenticationFilter { configuration }
    }

    pub fn is_request_authorized(
        &self,
        config: &ProxySection,
        req: &Request<Body>,
        remote_addr: &SocketAddr,
    ) -> Result<(), ProxyError> {
        let is_origin_allowed = self.is_origin_allowed(config, remote_addr);
        let is_authenticated = self.is_authenticated(config, req);

        let request_allowed = is_origin_allowed && is_authenticated;
        match request_allowed {
            true => Ok(()),
            false => Err(ProxyError::Forbidden),
        }
    }

    fn is_origin_allowed(&self, config: &ProxySection, remote_addr: &SocketAddr) -> bool {
        match config.allowed_origins.len() {
            0 => true,
            _ => config.allowed_origins.contains(&remote_addr.ip().to_string()),
        }
    }

    fn is_authenticated(&self, config: &ProxySection, req: &Request<Body>) -> bool {
        match &config.secret {
            None => true,
            Some(route_secret) => req
                .headers()
                .get(&self.configuration.server_section.authorization_header)
                .filter(|client_secret| client_secret.as_bytes() == route_secret.as_bytes())
                .is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
    use std::str::FromStr;

    use log::{error, info, Level};
    use regex::Regex;

    use crate::configuration::ServerSection;

    use super::*;

    extern crate log;
    extern crate simple_logger;

    #[test]
    fn filter_should_grant_if_public() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let req = Request::new(Body::empty());
        let remote_addr = "127.0.0.1:45220".parse().unwrap();
        let config = proxy_sections.get(0).unwrap();
        let is_authenticated = filter.is_request_authorized(config, &req, &remote_addr);

        assert_eq!(is_authenticated.is_ok(), true)
    }

    fn test_auth_filter() -> (AuthenticationFilter, Vec<ProxySection>) {
        let proxy_sections = vec![
            // Public section
            ProxySection {
                name: Some(String::from("section-1")),
                matching_path: String::from("/path-1"),
                matching_path_regex: Regex::new(&"/path-1").unwrap(),
                forward_to: String::from("http://localhost:9990"),
                secret: None,
                allowed_origins: vec![],
            },
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
                forward_to: String::from("http://localhost:10100"),
                secret: None,
                allowed_origins: vec![],
            },
        ];
        let config = Configuration {
            server_section: ServerSection {
                connection_string: String::from("127.0.0.1:8787"),
                authorization_header: String::from("Authorization"),
            },
            proxy_sections: proxy_sections.clone(),
        };

        let a_config = Arc::new(config);
        let filter = AuthenticationFilter::new(a_config);
        (filter, proxy_sections)
    }

    fn setup() {
        simple_logger::init();
    }
}

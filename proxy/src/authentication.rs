use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request};
use log::{error, info, Level};

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
        let origin_check = self.is_origin_allowed(config, remote_addr);
        let authentication_check = self.is_authenticated(config, req);

        let error = origin_check.err().or(authentication_check.err());
        match error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    fn is_origin_allowed(&self, config: &ProxySection, remote_addr: &SocketAddr) -> Result<(), ProxyError> {
        match config.allowed_origins.len() {
            0 => Ok(()),
            _ => match config.allowed_origins.contains(&remote_addr.ip().to_string()) {
                true => Ok(()),
                false => Err(ProxyError::BadOrigin),
            },
        }
    }

    fn is_authenticated(&self, config: &ProxySection, req: &Request<Body>) -> Result<(), ProxyError> {
        match &config.secret {
            None => Ok(()),
            Some(route_secret) => {
                let authenticated = req
                    .headers()
                    .get(&self.configuration.server_section.authorization_header)
                    .filter(|client_secret| client_secret.as_bytes() == route_secret.as_bytes())
                    .is_some();
                match authenticated {
                    true => Ok(()),
                    false => Err(ProxyError::BadAuthorization),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
    use std::str::FromStr;

    use hyper::http::HeaderValue;
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

    #[test]
    fn filter_should_deny_if_no_authorization_found() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let req = Request::new(Body::empty());
        let remote_addr = "127.0.0.1:45220".parse().unwrap();
        let config = proxy_sections.get(1).unwrap();
        let is_authenticated = filter.is_request_authorized(config, &req, &remote_addr);

        assert_eq!(is_authenticated.err().unwrap(), ProxyError::BadAuthorization)
    }

    #[test]
    fn filter_should_deny_if_authorization_is_wrong() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let mut req = Request::new(Body::empty());
        req.headers_mut()
            .append("Authorization", HeaderValue::from_str("poiuyt").unwrap());

        let remote_addr = "127.0.0.1:45220".parse().unwrap();
        let config = proxy_sections.get(1).unwrap();
        let is_authenticated = filter.is_request_authorized(config, &req, &remote_addr);

        assert_eq!(is_authenticated.err().unwrap(), ProxyError::BadAuthorization)
    }

    #[test]
    fn filter_should_grant_if_authorization_is_correct() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let mut req = Request::new(Body::empty());
        req.headers_mut()
            .append("Authorization", HeaderValue::from_str("abcde").unwrap());

        let remote_addr = "127.0.0.1:45220".parse().unwrap();
        let config = proxy_sections.get(1).unwrap();
        let is_authenticated = filter.is_request_authorized(config, &req, &remote_addr);

        assert_eq!(is_authenticated.is_ok(), true)
    }

    #[test]
    fn filter_should_deny_if_origin_is_wrong() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let req = Request::new(Body::empty());
        let remote_addr = "99.99.99.99:12345".parse().unwrap();
        let config = proxy_sections.get(2).unwrap();
        let is_authenticated = filter.is_request_authorized(config, &req, &remote_addr);

        assert_eq!(is_authenticated.err().unwrap(), ProxyError::BadOrigin)
    }

    #[test]
    fn filter_should_grant_if_origin_is_correct() {
        setup();
        let (filter, proxy_sections) = test_auth_filter();

        let req = Request::new(Body::empty());
        let remote_addr = "88.88.88.88:12345".parse().unwrap();
        let config = proxy_sections.get(2).unwrap();
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
            // With credentials
            ProxySection {
                name: Some(String::from("section-1")),
                matching_path: String::from("/path-1"),
                matching_path_regex: Regex::new(&"/path-1").unwrap(),
                forward_to: String::from("http://localhost:9990"),
                secret: Some(String::from("abcde")),
                allowed_origins: vec![],
            },
            // With origin
            ProxySection {
                name: Some(String::from("section-2")),
                matching_path: String::from("/path-2"),
                matching_path_regex: Regex::new(&"/path-2").unwrap(),
                forward_to: String::from("http://localhost:10100"),
                secret: None,
                allowed_origins: vec![String::from("127.0.0.1"), String::from("88.88.88.88")],
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

use std::borrow::BorrowMut;
use std::hash::Hash;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use futures::future;
use futures::future::IntoFuture;
use futures::stream::Stream;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::header::HeaderValue;
use hyper::http::uri::InvalidUri;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Error, Request, Response, StatusCode, Uri};
use log::error;
use log::info;
use serde_json::json;

use crate::authentication_filter::AuthenticationFilter;
use crate::configuration::{Configuration, ProxySection};

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Proxy {
    configuration: Arc<Configuration>,
    client: Client<HttpConnector<GaiResolver>, Body>,
    authentication_filter: AuthenticationFilter,
    remote_addr: SocketAddr,
}

#[derive(Debug, PartialEq)]
pub enum ProxyError {
    BadAuthorization,
    BadOrigin,
    Forbidden,
    InvalidTargetUrl,
    NoTargetFound,
}

impl Service for Proxy {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = BoxFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let proxy_config = self.configuration.section_from_uri(req.uri());
        match proxy_config {
            Some(config) => {
                match self
                    .authentication_filter
                    .is_request_authorized(&config, &req, &self.remote_addr)
                {
                    Ok(_) => self.proxy_request(req, config),
                    Err(err) => self.error_response(err),
                }
            }
            None => self.error_response(ProxyError::NoTargetFound),
        }
    }
}

impl Proxy {
    pub fn new(configuration: Arc<Configuration>, authentication_filter: AuthenticationFilter, remote_addr: SocketAddr) -> Self {
        Proxy {
            configuration: configuration.clone(),
            client: Client::new(),
            authentication_filter,
            remote_addr,
        }
    }

    fn proxy_request(&self, original_req: Request<Body>, config: ProxySection) -> BoxFuture {
        let target_uri = self.get_target_uri(original_req.uri(), config);
        match target_uri {
            Ok(uri) => {
                let (parts, body) = original_req.into_parts();
                let mut proxy_req = Request::from_parts(parts, body);

                info!("Proxying to: {}", uri.to_string());
                *proxy_req.uri_mut() = uri;

                let forward_header = HeaderValue::from_bytes(self.remote_addr.ip().to_string().as_bytes()).unwrap();
                (*proxy_req.headers_mut()).append("X-Forwarded-For", forward_header);

                let res = self.client.request(proxy_req);
                return Box::new(res);
            }
            Err(err) => self.error_response(err),
        }
    }

    fn get_target_uri(&self, original_uri: &Uri, config: ProxySection) -> Result<Uri, ProxyError> {
        let path = original_uri.path();
        let query = original_uri.query().map(|q| format!("?{}", q)).unwrap_or(String::from(""));
        let target = format!("{}{}{}", config.forward_to, path, query);

        Uri::from_str(&target).or_else(|invalid_uri| {
            error!("Invalid target url: {}", invalid_uri);
            Err(ProxyError::InvalidTargetUrl)
        })
    }

    fn error_response(&self, proxy_error: ProxyError) -> BoxFuture {
        let (status, error_message) = match proxy_error {
            ProxyError::BadAuthorization => (StatusCode::FORBIDDEN, "Bad authorization"),
            ProxyError::BadOrigin => (StatusCode::FORBIDDEN, "Bad origin"),
            ProxyError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            ProxyError::InvalidTargetUrl => (StatusCode::INTERNAL_SERVER_ERROR, "Invalid target url"),
            ProxyError::NoTargetFound => (StatusCode::NOT_FOUND, "Not found"),
        };

        let response_body = json!({ "message": error_message }).to_string();
        Box::new(future::ok(
            Response::builder().status(status).body(Body::from(response_body)).unwrap(),
        ))
    }
}

impl IntoFuture for Proxy {
    type Future = future::FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = hyper::Error;

    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
    use std::str::FromStr;

    use log::{error, info, Level};
    use regex::Regex;

    use crate::configuration::ServerSection;

    use super::*;

    extern crate log;
    extern crate simple_logger;

    #[test]
    fn should_create_correct_target_uri_with_query() {
        setup();
        let (proxy, config) = test_proxy();

        let url = &Uri::from_str(&"http://domain-1.net/path-2/sub-path-2?query_arg1=val1&query_arg1=val1").unwrap();
        let result = proxy.get_target_uri(url, config);
        assert_eq!(
            result.unwrap(),
            "http://localhost:10100/path-2/sub-path-2?query_arg1=val1&query_arg1=val1"
        );
    }

    #[test]
    fn should_create_correct_target_uri_without_query() {
        setup();
        let (proxy, config) = test_proxy();

        let url = &Uri::from_str(&"http://domain-1.net/path-2/sub-path-2").unwrap();
        let result = proxy.get_target_uri(url, config);
        assert_eq!(result.unwrap(), "http://localhost:10100/path-2/sub-path-2");
    }

    fn setup() {
        simple_logger::init();
    }

    fn test_proxy() -> (Proxy, ProxySection) {
        let proxy_section = ProxySection {
            name: Some(String::from("section-2")),
            matching_path: String::from("/path-2"),
            matching_path_regex: Regex::new(&"/path-2").unwrap(),
            forward_to: String::from("http://localhost:10100"),
            secret: None,
            allowed_origins: vec![],
        };
        let config = Configuration {
            server_section: ServerSection {
                connection_string: String::from("127.0.0.1:8787"),
                authorization_header: String::from("Authorization"),
            },
            proxy_sections: vec![proxy_section.clone()],
        };

        let a_config = Arc::new(config);
        let proxy = Proxy {
            configuration: a_config.clone(),
            client: Client::new(),
            authentication_filter: AuthenticationFilter::new(a_config.clone()),
            remote_addr: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 43522),
        };
        (proxy, proxy_section)
    }
}

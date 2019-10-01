use std::borrow::BorrowMut;
use std::hash::Hash;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use futures::future;
use futures::future::IntoFuture;
use futures::stream::Stream;
use hyper::{Body, Client, Error, Request, Response, StatusCode, Uri};
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::header::HeaderValue;
use hyper::http::uri::InvalidUri;
use hyper::rt::Future;
use hyper::service::Service;
use log::error;
use log::info;
use serde_json::json;

use crate::authentication::AuthenticationFilter;
use crate::config_resolver::ProxyConfigResolver;
use crate::configuration::{Configuration, ProxySection};

type BoxFuture = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

pub struct Proxy {
    configuration: Arc<Configuration>,
    config_resolver: ProxyConfigResolver,
    client: Client<HttpConnector<GaiResolver>, Body>,
    authentication_filter: AuthenticationFilter,
    remote_addr: SocketAddr,
}

enum ProxyError {
    Forbidden,
    NoTargetFound,
    InvalidTargetUrl,
}

impl Service for Proxy {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = BoxFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        info!("Proxying request: {:#?}", req);

        match self.authentication_filter.is_request_authorized(&req) {
            Ok(_) => self.proxy_request(req),
            Err(message) =>
                self.error_response(ProxyError::Forbidden),
        }
    }
}

impl Proxy {
    pub fn new(
        configuration: Arc<Configuration>,
        authentication_filter: AuthenticationFilter,
        remote_addr: SocketAddr,
    ) -> Self {
        Proxy {
            configuration: configuration.clone(),
            config_resolver: ProxyConfigResolver::new(configuration),
            client: Client::new(),
            authentication_filter,
            remote_addr,
        }
    }

    fn proxy_request(&self, original_req: Request<Body>) -> BoxFuture {
        let target_uri = self.get_proxy_uri(original_req.uri());
        match target_uri {
            Ok(uri) => {
                let (parts, body) = original_req.into_parts();
                let mut proxy_req = Request::from_parts(parts, body);

                *proxy_req.uri_mut() = uri;

                let forward_header =
                    HeaderValue::from_bytes(self.remote_addr.ip().to_string().as_bytes()).unwrap();
                (*proxy_req.headers_mut()).append("X-Forwarded-For", forward_header);

                info!("Sending request: {:#?}", proxy_req);

                let res = self.client.request(proxy_req);

                return Box::new(res);
            }
            Err(err) => self.error_response(err)
        }
    }

    fn get_proxy_uri(&self, original_uri: &Uri) -> Result<Uri, ProxyError> {
        let config = self.config_resolver.section_from_uri(original_uri);
        match config {
            Some(c) => {
                let path = original_uri.path();
                let query = original_uri.query().map(|q| format!("?{}", q)).unwrap_or(String::from(""));
                let target = format!("{}{}{}", c.forward_to, path, query);

                Uri::from_str(&target)
                    .or_else(|invalid_uri| Err(ProxyError::InvalidTargetUrl))
            }
            None => Err(ProxyError::NoTargetFound)
        }
    }

    fn error_response(&self, error: ProxyError) -> BoxFuture {
        let (status, error_message) = match error {
            ProxyError::NoTargetFound => (StatusCode::FORBIDDEN, "Cannot proxy request"),
            ProxyError::InvalidTargetUrl => (StatusCode::INTERNAL_SERVER_ERROR, "Cannot proxy request"),
            ProxyError::Forbidden => (StatusCode::INTERNAL_SERVER_ERROR, "Forbidden"),
        };

        let response_body = json!({"message": error_message}).to_string();
        Box::new(
            future::ok(
                Response::builder()
                    .status(status)
                    .body(Body::from(response_body))
                    .unwrap())
        )
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
    fn should_create_correct_proxy_uri_with_query() {
        setup();
        let proxy = test_proxy();

        let with_query = proxy.get_proxy_uri(
            &Uri::from_str(&"http://domain-1.net/path-2/sub-path-2?query_arg1=val1&query_arg1=val1").unwrap(),
        );
        assert_eq!(with_query.is_ok(), true);
        assert_eq!(with_query.unwrap(), "http://localhost:10100/path-2/sub-path-2?query_arg1=val1&query_arg1=val1");
    }

    #[test]
    fn should_create_correct_proxy_uri_without_query() {
        setup();
        let proxy = test_proxy();

        let with_query = proxy.get_proxy_uri(
            &Uri::from_str(&"http://domain-1.net/path-2/sub-path-2").unwrap(),
        );
        assert_eq!(with_query.is_ok(), true);
        assert_eq!(with_query.unwrap(), "http://localhost:10100/path-2/sub-path-2");
    }

    #[test]
    fn should_return_err_if_no_proxy_config_match() {
        setup();
        let proxy = test_proxy();

        let with_query = proxy.get_proxy_uri(
            &Uri::from_str(&"http://domain-1.net/should-not-match").unwrap(),
        );
        assert_eq!(with_query.is_ok(), false);
    }

    fn setup() {
        simple_logger::init();
    }

    fn test_proxy() -> Proxy {
        let config = Configuration {
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
                    allowed_origins: None,
                },
                ProxySection {
                    name: Some(String::from("section-2")),
                    matching_path: String::from("/path-2"),
                    matching_path_regex: Regex::new(&"/path-2").unwrap(),
                    forward_to: String::from("http://localhost:10100"),
                    secret: None,
                    allowed_origins: None,
                },
            ],
        };

        let a_config = Arc::new(config);
        Proxy {
            configuration: a_config.clone(),
            config_resolver: ProxyConfigResolver::new(a_config.clone()),
            client: Client::new(),
            authentication_filter: AuthenticationFilter::new(a_config.clone()),
            remote_addr: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 43522),
        }
    }
}

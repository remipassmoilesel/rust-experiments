use std::net::SocketAddr;
use std::sync::Arc;

use futures::future;
use futures::future::IntoFuture;
use futures::stream::Stream;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::header::HeaderValue;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Error, Request, Response, Uri};
use log::error;
use log::info;

use crate::authentication::AuthenticationFilter;
use crate::config_resolver::ProxyConfigResolver;
use crate::configuration::{Configuration, ProxySection};
use hyper::http::uri::InvalidUri;
use std::borrow::BorrowMut;
use std::str::FromStr;

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Proxy {
    configuration: Arc<Configuration>,
    config_resolver: ProxyConfigResolver,
    client: Client<HttpConnector<GaiResolver>, Body>,
    authentication_filter: AuthenticationFilter,
    remote_addr: SocketAddr,
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
            Err(reason) => self.deny_request(&req, reason),
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
        let target_uri = self.get_proxy_uri(original_req.uri()).unwrap();
        let (parts, body) = original_req.into_parts();
        let mut proxy_req = Request::from_parts(parts, body);

        *proxy_req.uri_mut() = target_uri;

        let forward_header =
            HeaderValue::from_bytes(self.remote_addr.ip().to_string().as_bytes()).unwrap();
        (*proxy_req.headers_mut()).append("X-Forwarded-For", forward_header);

        info!("Sending request: {:#?}", proxy_req);

        let res = self.client.request(proxy_req);

        return Box::new(res);
    }

    fn deny_request(&self, req: &Request<Body>, reason: String) -> BoxFuture {
        let response = Response::new(Body::empty());
        Box::new(future::ok(response))
    }

    fn get_proxy_uri(&self, original_uri: &Uri) -> Result<Uri, InvalidUri> {
        let proxy_section = self.config_resolver.section_from_uri(original_uri).unwrap();
        let path = original_uri.path();
        let query = original_uri.query().unwrap_or("");
        let target = format!("{}{}{}", proxy_section.forward_to, path, query);

        Uri::from_str(&target)
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

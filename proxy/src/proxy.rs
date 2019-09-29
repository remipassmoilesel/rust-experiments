use std::net::SocketAddr;
use std::sync::Arc;

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::header::HeaderValue;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request, Response, Uri};
use log::error;
use log::info;

use crate::authentication::AuthenticationFilter;
use crate::configuration::Configuration;

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Proxy {
    configuration: Arc<Configuration>,
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
            Ok(_) => self.proxy_request(&req),
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
            configuration,
            client: Client::new(),
            authentication_filter,
            remote_addr,
        }
    }

    fn proxy_request(&self, req: &Request<Body>) -> BoxFuture {
        // TODO: add better uri
        let url: Uri = "http://httpbin.org/response-headers?foo=bar"
            .parse()
            .unwrap();

        let mut request_builder = Request::builder();
        request_builder.method(req.method());
        request_builder.uri(url);
        req.headers().iter().for_each(|(name, value)| {
            request_builder.header(name, value);
        });
        match HeaderValue::from_bytes(self.remote_addr.to_string().as_bytes()) {
            Ok(header_value) => {
                req.headers().append("X-Forwarded-For", header_value);
            }
            Err(err) => {
                error!("Remove address is invalid: {} {:?}", self.remote_addr, err);
            }
        }
        let proxy_request = request_builder.body(*req.body());

        let request_result = match proxy_request {
            Ok(mut req) => self.client.request(req), // TODO: cast to future
            Err(e) => future::err(e),
        };

        Box::new(request_result)
    }

    fn deny_request(&self, req: &Request<Body>, reason: String) -> BoxFuture {
        let response = Response::new(Body::empty());
        Box::new(future::ok(response))
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

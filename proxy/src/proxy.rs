use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request, Response};

use crate::configuration::Configuration;

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Proxy {
    configuration: Configuration,
    client: Client<HttpConnector<GaiResolver>, Body>,
}

impl Service for Proxy {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = BoxFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        Box::new(future::ok(Response::new(Body::empty())))
    }
}

impl Proxy {
    pub fn new(configuration: Configuration) -> Self {
        Proxy {
            configuration,
            client: Client::new(),
        }
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

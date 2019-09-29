use std::sync::Arc;

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request, Response, Uri};
use log::info;

use crate::authentication_filter::AuthenticationFilter;
use crate::configuration::Configuration;

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Proxy {
    configuration: Arc<Configuration>,
    client: Client<HttpConnector<GaiResolver>, Body>,
    authentication_filter: AuthenticationFilter,
}

impl Service for Proxy {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = BoxFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        info!("Proxying request: {:#?}", req);

        match self.authentication_filter.is_request_authorized(&req) {
            Ok(_) => {
                let url: Uri = "http://httpbin.org/response-headers?foo=bar"
                    .parse()
                    .unwrap();
                info!("{:#?}", url);

                let request_result = self.client
                    .get(url)
                    .map(|res| {
                        info!("Response: {:#?}", res);
                        Response::new(Body::empty())
                    })
                    .map_err(|err| {
                        info!("Error: {:#?}", err);
                        err
                    });

                Box::new(request_result)
            }
            _ => Box::new(future::ok(Response::new(Body::empty()))),
        }
    }
}

impl Proxy {
    pub fn new(
        configuration: Arc<Configuration>,
        authentication_filter: AuthenticationFilter,
    ) -> Self {
        Proxy {
            configuration,
            client: Client::new(),
            authentication_filter,
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

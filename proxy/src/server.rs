extern crate futures;
extern crate log;

use std::error::Error;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use futures::future;
use hyper::http;
use hyper::rt::{self, Future, Stream};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn, service_fn_ok};
use hyper::{Body, Request, Response, Server as HyperServer, StatusCode};
use hyper::{Client, Uri};
use log::{error, info};
use tokio_core::reactor::Core;

use crate::authentication_filter::AuthenticationFilter;
use crate::configuration::Configuration;
use crate::proxy::Proxy;

pub struct Server {
    configuration: Arc<Configuration>,
}

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

impl Server {
    pub(crate) fn new(configuration: Configuration) -> Server {
        Server {
            configuration: Arc::new(configuration),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = match SocketAddr::from_str(&self.configuration.server_section.connection_string)
        {
            Ok(a) => a,
            Err(e) => return Err(Box::new(e)),
        };

        let config = self.configuration.clone();
        let new_svc = make_service_fn(move |_socket: &AddrStream| {
            let filter = AuthenticationFilter::new(config.clone());
            Proxy::new(config.clone(), filter)
        });

        match HyperServer::try_bind(&addr) {
            Ok(builder) => {
                let server = builder
                    .serve(new_svc)
                    .map_err(|e| error!("Server error: {}", e));
                hyper::rt::run(server);
            }
            Err(e) => return Err(Box::new(e)),
        }

        Ok(()) // TODO: improve return value
    }
}

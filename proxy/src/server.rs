extern crate futures;

use std::error::Error;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::str::FromStr;

use futures::future;
use hyper::{Body, Request, Response, Server as HyperServer, StatusCode};
use hyper::{Client, Uri};
use hyper::http;
use hyper::rt::{self, Future, Stream};
use hyper::service::{service_fn, service_fn_ok};
use tokio_core::reactor::Core;

use crate::configuration::Configuration;
use crate::proxy::Proxy;

pub struct Server {
    configuration: Configuration,
    proxy: Proxy,
}

type BoxFuture = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

impl Server {
    pub(crate) fn new(configuration: Configuration, proxy: Proxy) -> Server {
        Server {
            configuration,
            proxy,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = match SocketAddr::from_str(&self.configuration.server_section.connection_string) {
            Ok(a) => a,
            Err(e) => return Err(Box::new(e)),
        };

        let new_svc = || service_fn(Server::handle_requests);

        match HyperServer::try_bind(&addr) {
            Ok(builder) => {
                let server = builder
                    .serve(new_svc)
                    .map_err(|e| eprintln!("Server error: {}", e));
                hyper::rt::run(server);
            }
            Err(e) => return Err(Box::new(e)),
        }

        Ok(()) // TODO: improve return value
    }

    // TODO: move functions to a dedicated struct
    fn handle_requests(req: Request<Body>) -> BoxFuture {
        println!("Proxying request: {:#?}", req);
        let client = Client::new();

        match Server::is_request_authorized(&req) {
            Ok(_) => {
                let url: Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
                println!("{:#?}", url);

                let request_result = client
                    .get(url)
                    .map(|res| {
                        println!("Response: {:#?}", res);
                        Response::new(Body::empty())
                    })
                    .map_err(|err| {
                        println!("Error: {:#?}", err);
                        err
                    });

                Box::new(request_result)
            }
            _ => {
                Box::new(future::ok(Response::new(Body::empty())))
            }
        }
    }

    fn is_request_authorized(req: &Request<Body>) -> Result<(), String> {
        // Err(String::from("Not allowed"))
        Ok(())
    }
}

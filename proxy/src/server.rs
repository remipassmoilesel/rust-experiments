use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::{Body, Request, Response, Server as HyperServer};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use crate::configuration::Configuration;
use crate::proxy::Proxy;

pub struct Server {
    configuration: Configuration,
    proxy: Proxy,
}

impl Server {
    pub(crate) fn new(configuration: Configuration, proxy: Proxy) -> Server {
        Server { configuration, proxy }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = match SocketAddr::from_str(&self.configuration.server_section.connection_string) {
            Ok(a) => a,
            Err(e) => return Err(Box::new(e))
        };

        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        let new_svc = || {
            // service_fn_ok converts our function into a `Service`
            service_fn_ok(Server::hello_world)
        };

        let server = HyperServer::bind(&addr)
            .serve(new_svc)
            .map_err(|e| eprintln!("server error: {}", e));

        // Run this server for... forever!
        hyper::rt::run(server);
        Ok(())
    }

    fn hello_world(req: Request<Body>) -> Response<Body> {
        println!("{:#?}", req);
        Response::new(Body::from("PHRASE"))
    }
}

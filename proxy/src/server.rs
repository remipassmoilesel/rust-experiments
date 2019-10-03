extern crate futures;
extern crate log;

use std::error::Error;

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use hyper::rt::Future;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::Server as HyperServer;

use log::error;

use crate::authentication::AuthenticationFilter;
use crate::configuration::Configuration;
use crate::proxy::Proxy;

pub struct Server {
    configuration: Arc<Configuration>,
}

impl Server {
    pub(crate) fn new(configuration: Configuration) -> Server {
        Server {
            configuration: Arc::new(configuration),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = match SocketAddr::from_str(&self.configuration.server_section.connection_string) {
            Ok(a) => a,
            Err(e) => return Err(Box::new(e)),
        };

        let config = self.configuration.clone();
        let service_factory = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let filter = AuthenticationFilter::new(config.clone());
            Proxy::new(config.clone(), filter, remote_addr)
        });

        match HyperServer::try_bind(&addr) {
            Ok(builder) => {
                let server = builder.serve(service_factory).map_err(|e| error!("Server error: {}", e));
                hyper::rt::run(server);
            }
            Err(e) => return Err(Box::new(e)),
        }

        Ok(()) // TODO: improve return value
    }
}

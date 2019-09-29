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

use crate::authentication_filter::AuthenticationFilter;
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

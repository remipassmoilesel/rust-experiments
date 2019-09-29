use std::error::Error;

use hyper::{Body, Request, Response};

use crate::proxy::Proxy;

pub struct Server {
    proxy: Proxy,
}

impl Server {
    pub(crate) fn new(proxy: Proxy) -> Server {
        Server { proxy }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    fn hello_world(req: Request<Body>) -> Response<Body> {
        println!("{:#?}", req);
        Response::new(Body::from("PHRASE"))
    }
}

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::error::Error;
use std::io::{self, Write};

use hyper::{Body, Request};
use hyper::{Client, Uri};
use hyper::rt::{self, Future, Stream};
use tokio_core::reactor::Core;

use crate::configuration::Configuration;

pub struct Proxy {
    configuration: Configuration,
}

impl Proxy {
    pub fn new(configuration: Configuration) -> Proxy {
        Proxy { configuration }
    }

    pub fn forward_request(req: &Request<Body>) {
        ()
    }
}

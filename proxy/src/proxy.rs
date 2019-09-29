use std::error::Error;

use crate::configuration::Configuration;
use std::rc::Rc;

pub struct Proxy {
    configuration: Configuration,
}

impl Proxy {
    pub fn new(configuration: Configuration) -> Proxy {
        Proxy {
            configuration: configuration,
        }
    }
}

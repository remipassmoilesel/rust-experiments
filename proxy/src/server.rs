use crate::proxy::Proxy;
use std::error::Error;

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
}

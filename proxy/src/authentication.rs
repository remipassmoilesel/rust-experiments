use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request};

use crate::configuration::{Configuration, ProxySection};
use crate::proxy::ProxyError;

pub struct AuthenticationFilter {
    configuration: Arc<Configuration>,
}

impl AuthenticationFilter {
    pub fn new(configuration: Arc<Configuration>) -> AuthenticationFilter {
        AuthenticationFilter { configuration }
    }

    pub fn is_request_authorized(
        &self,
        req: &Request<Body>,
        config: &ProxySection,
        remote_addr: &SocketAddr,
    ) -> Result<(), ProxyError> {
        let is_origin_allowed = self.is_origin_allowed(config, remote_addr);

        let request_allowed = is_origin_allowed;
        match request_allowed {
            true => Ok(()),
            false => Err(ProxyError::Forbidden),
        }
    }

    fn is_origin_allowed(&self, config: &ProxySection, remote_addr: &SocketAddr) -> bool {
        match config.allowed_origins.len() {
            0 => true,
            _ => config.allowed_origins.contains(&remote_addr.ip().to_string()),
        }
    }
}

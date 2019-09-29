use std::sync::Arc;

use hyper::{Body, Request};

use crate::configuration::Configuration;

pub struct AuthenticationFilter {
    configuration: Arc<Configuration>,
}

impl AuthenticationFilter {
    pub fn new(configuration: Arc<Configuration>) -> AuthenticationFilter {
        AuthenticationFilter { configuration }
    }

    pub fn is_request_authorized(&self, req: &Request<Body>) -> Result<(), String> {
        Ok(())
    }
}

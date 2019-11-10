use hyper::{Body, Request};
use std::sync::Arc;

use crate::create_request_handler;

use super::PresentationError;
use crate::application::ResponseFuture;
use crate::service::{LoginService, SimpleLoginService};

pub struct LoginController {
    login_service: Arc<dyn LoginService>,
}

impl LoginController {
    pub fn new() -> Result<LoginController, PresentationError> {
        let controller = LoginController {
            login_service: Arc::new(SimpleLoginService::new()?),
        };
        Ok(controller)
    }

    pub fn signup(&self, request: Request<Body>) -> ResponseFuture {
        create_request_handler!(request, self.login_service, signup)
    }

    pub fn signin(&self, request: Request<Body>) -> ResponseFuture {
        create_request_handler!(request, self.login_service, signin)
    }
}

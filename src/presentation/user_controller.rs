use hyper::{Body, Request};
use std::sync::Arc;

use crate::create_request_handler;

use super::PresentationError;
use crate::application::ResponseFuture;
use crate::service::{LoginService, SimpleLoginService, SimpleUserService, UserService};

pub struct UserController {
    login_service: Arc<dyn LoginService>,
	user_service: Arc<dyn UserService>
}

impl UserController {
    pub fn new() -> Result<UserController, PresentationError> {
        let controller = UserController {
            login_service: Arc::new(SimpleLoginService::new()?),
			user_service: Arc::new(SimpleUserService::new()?)
        };
        Ok(controller)
    }

    pub fn signup(&self, request: Request<Body>) -> ResponseFuture {
        create_request_handler!(request, self.login_service, signup)
    }

    pub fn signin(&self, request: Request<Body>) -> ResponseFuture {
        create_request_handler!(request, self.login_service, signin)
    }

	pub fn get_session_user(&self, request: Request<Body>) -> ResponseFuture {
		create_request_handler!(request, self.user_service, get_user_by_session)
	}
}

use hyper::{Body, Method, Request};

use super::{static_response, ApplicationError, ResponseFuture};
use crate::presentation::UserController;

pub struct Application {
    user_controller: UserController,
}

impl Application {
    pub fn new() -> Result<Application, ApplicationError> {
        let app = Application {
            user_controller: UserController::new()?,
        };

        Ok(app)
    }

    pub fn request(&self, request: Request<Body>) -> ResponseFuture {
        info!(
            "Incoming request: method = '{}', uri = '{}'",
            request.method(),
            request.uri()
        );
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => static_response::error_405_future(),
        }
    }

    fn handle_get(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            _ => static_response::error_404_future(),
        }
    }

    fn handle_post(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/api/signup" => self.user_controller.signup(request),
            "/api/signin" => self.user_controller.signin(request),
            "/api/sessionuser" => self.user_controller.get_session_user(request),
            _ => static_response::error_404_future(),
        }
    }
}

use hyper::{Body, Method, Request};

use super::{static_response, ApplicationError, ResponseFuture};
use crate::presentation::LoginController;

pub struct Application {
    login_controller: LoginController,
}

impl Application {
    pub fn new() -> Result<Application, ApplicationError> {
        let app = Application {
            login_controller: LoginController::new()?,
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
            "/api/signup" => self.login_controller.signup(request),
            "/api/signin" => self.login_controller.signin(request),
            _ => static_response::error_404_future(),
        }
    }
}

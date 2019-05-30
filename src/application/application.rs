use std::net::SocketAddr;
use hyper::{ Body, Response, Request, Method };
use hyper::service::Service;
use futures::{ future, Future };

use crate::presentation::LoginController;
use super::ApplicationError;



pub struct Application {
    login_controller: LoginController
}

impl Application {
    pub fn new() -> Result<Application, ApplicationError> {
        let app = Application {
            login_controller: LoginController::new()?
        };
        Ok(app)
    }

    pub fn request(&self, request: Request<Body>) -> Response<Body> {
        info!("Request: HTTP {} {}", request.method(), request.uri());
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => respond_404()
        }
    }

    fn handle_get(&self, request: Request<Body>) -> Response<Body> {
        respond_404()
    }

    fn handle_post(&self, request: Request<Body>) -> Response<Body> {
        match request.uri().path() {
            "/signup" => Response::default(),
            _ => respond_404()
        }
    }
}

fn respond_404() -> Response<Body> {
    Response::builder()
        .status(404)
        .body(Body::from("Page not found"))
        .unwrap()
}

use std::net::SocketAddr;
use hyper::{ Body, Response, Request, Server };
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
        info!("Recv request!");
        Response::builder()
            .status(200)
            .body(Body::from("Sers"))
            .unwrap()
    }
}

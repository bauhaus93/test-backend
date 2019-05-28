use std::net::SocketAddr;
use hyper::{ Body, Response, Request, Server };
use hyper::service::Service;
use futures::{ future, Future };

use super::ApplicationError;


pub struct Application {

}

impl Application {
    pub fn new() -> Result<Application, ApplicationError> {
        Ok(Application{})
    }

    pub fn request(&self, request: Request<Body>) -> Response<Body> {
        info!("Recv request!");
        Response::builder()
            .status(200)
            .body(Body::from("Sers"))
            .unwrap()
    }
}
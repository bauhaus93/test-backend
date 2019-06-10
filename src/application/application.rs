use hyper::{ Body, Response, Request, Method, StatusCode, header };
use futures::{ Future, Stream };

use crate::presentation::LoginController;
use crate::dto::Login;
use super::{ ApplicationError, ResponseFuture, respond_404, respond_500 };

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

    pub fn request(&self, request: Request<Body>) -> ResponseFuture {
        info!("Request: HTTP {} {}", request.method(), request.uri());
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => respond_404()
        }
    }

    fn handle_get(&self, request: Request<Body>) -> ResponseFuture{
        respond_404()
    }

    fn handle_post(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/signup" => {
                Box::new(
                    request.into_body()
                    .concat2()
                    .from_err()
                    .and_then(|body| {
                        let content: Login = serde_json::from_slice(&body.to_vec())?;
                        info!("Content = {}", content);

                        let response = Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "text")
                            .body(Body::from("LELEL"))?;
                        Ok(response)
                    }))
            },
            _ => respond_404()
        }
    }
}
use std::collections::BTreeMap;
use hyper::{ Body, Response, Request, Method, StatusCode, header };
use futures::{ Future, future, Stream };

use crate::presentation::LoginController;
use crate::dto::Login;
use super::{ ApplicationError, ResponseFuture, StaticResponse };

pub struct Application {
    static_responses: BTreeMap<String, StaticResponse>,
    login_controller: LoginController
}

impl Application {
    pub fn new() -> Result<Application, ApplicationError> {
        let mut app = Application {
            static_responses: BTreeMap::new(),
            login_controller: LoginController::new()?
        };
        app.load_responses()?;

        Ok(app)
    }

    fn load_responses(&mut self) -> Result<(), ApplicationError> {
        info!("Loading static responses...");
        self.load_response("index", "assets/index.html", StatusCode::OK, "text/html")?;
        self.load_response("page-not-found", "assets/pagenotfound.html", StatusCode::NOT_FOUND, "text/html")?;
        info!("All static responses successfully loaded!");
        Ok(())
    }

    fn load_response(&mut self, name: &str, file_path: &str, status_code: StatusCode, content_type: &str) -> Result<(), ApplicationError> {
        info!("Loading static response: name = '{}', status code = '{}', content type = '{}', source = '{}'", name, status_code, content_type, file_path);
        let static_response = StaticResponse::from_file(file_path, status_code, content_type)?;
        self.static_responses.insert(name.into(), static_response);
        Ok(())
    }

    fn create_static_response(&self, name: &str) -> ResponseFuture {
        trace!("Creating static response instance: '{}'", name);
        match self.static_responses.get(name) {
            Some(r) => r.create_instance(),
            None => {
                warn!("Response '{}' not in map!", name);
                StaticResponse::fallback_500()
            }
        }
    }

    pub fn request(&self, request: Request<Body>) -> ResponseFuture {
        info!("Request: method = {}, uri = '{}'", request.method(), request.uri());
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => self.create_static_response("page-not-found")
        }
    }

    fn handle_get(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/" => self.create_static_response("index"),
            _ => self.create_static_response("page-not-found")
        }
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
            _ => self.create_static_response("page-not-found")
        }
    }
}
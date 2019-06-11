use std::collections::BTreeMap;
use hyper::{ Body, Response, Request, Method, StatusCode, header };
use futures::{ Future, Stream };

use crate::presentation::LoginController;
use crate::dto::Login;
use super::{ ApplicationError, ResponseFuture, StaticResponse, load_assets };

pub struct Application {
    asset_map: BTreeMap<String, StaticResponse>,
    login_controller: LoginController
}

impl Application {
    pub fn new(asset_folder: &str) -> Result<Application, ApplicationError> {
        let app = Application {
            asset_map: load_assets(asset_folder)?,
            login_controller: LoginController::new()?
        };

        Ok(app)
    }

    fn create_asset_response(&self, name: &str) -> ResponseFuture {
        trace!("Creating response from asset: '{}'", name);
        match self.asset_map.get(name) {
            Some(r) => r.create_instance(),
            None => {
                error!("Missing asset: '{}'", name);
                StaticResponse::fallback_500()
            }
        }
    }

    pub fn request(&self, request: Request<Body>) -> ResponseFuture {
        info!("Request: method = '{}', uri = '{}'", request.method(), request.uri());
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => StaticResponse::fallback_404()
        }
    }

    fn handle_get(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/" => self.create_asset_response("index-html"),
            "/custom.css" => self.create_asset_response("custom-css"),
            "/main.js" => self.create_asset_response("main-js"),
            _ => StaticResponse::fallback_404()
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
            _ => StaticResponse::fallback_404()
        }
    }
}
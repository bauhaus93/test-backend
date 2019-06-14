use std::collections::BTreeMap;
use hyper::{ Body, Request, Method };

use crate::presentation::LoginController;
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

    pub fn request(&self, request: Request<Body>) -> ResponseFuture {
        info!("Incoming request: method = '{}', uri = '{}'", request.method(), request.uri());
        match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => StaticResponse::error_405_future()
        }
    }

    fn create_response_from_asset(&self, name: &str) -> ResponseFuture {
        trace!("Creating response from asset: '{}'", name);
        match self.asset_map.get(name) {
            Some(r) => r.create_instance_future(),
            None => {
                error!("Asset {} not existing!", name);
                StaticResponse::error_500_future()
            }
        }
    }

    fn handle_get(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/" => self.create_response_from_asset("index-html"),
            "/custom.css" => self.create_response_from_asset("custom-css"),
            "/main.js" => self.create_response_from_asset("main-js"),
            _ => StaticResponse::error_404_future()
        }
    }

    fn handle_post(&self, request: Request<Body>) -> ResponseFuture {
        match request.uri().path() {
            "/signup" => self.login_controller.signup(request),
            _ => StaticResponse::error_404_future()
        }
    }
}
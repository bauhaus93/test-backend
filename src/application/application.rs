use std::collections::BTreeMap;
use hyper::{ Body, Response, Request, Method };
use futures::future;

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
        let result = match *request.method() {
            Method::GET => self.handle_get(request),
            Method::POST => self.handle_post(request),
            _ => Ok(StaticResponse::error_405())
        };
        let response = result_to_response(result);
        info!("Responding with status code = '{}'", response.status());
        create_response_future(response)
    }

    fn create_response_from_asset(&self, name: &str) -> Result<Response<Body>, ApplicationError> {
        trace!("Creating response from asset: '{}'", name);
        match self.asset_map.get(name) {
            Some(r) => r.create_instance(),
            None => Err(ApplicationError::AssetNotExisting(name.to_owned()))
        }
    }

    fn handle_get(&self, request: Request<Body>) -> Result<Response<Body>, ApplicationError> {
        match request.uri().path() {
            "/" => self.create_response_from_asset("index-html"),
            "/custom.css" => self.create_response_from_asset("custom-css"),
            "/main.js" => self.create_response_from_asset("main-js"),
            _ => Ok(StaticResponse::error_404())
        }
    }

    fn handle_post(&self, request: Request<Body>) -> Result<Response<Body>, ApplicationError> {
        match request.uri().path() {
            "/signup" => {
                upgrade_error(self.login_controller.signup(request))
            },
            _ => Ok(StaticResponse::error_404())
        }
    }
}

fn result_to_response<E: std::error::Error>(result: Result<Response<Body>, E>) -> Response<Body> {
    match result {
        Ok(response) => response,
        Err(e) => {
            error!("{}", e);
            StaticResponse::error_500()
        }
    }
}

fn create_response_future(response: Response<Body>) -> ResponseFuture {
    Box::new(future::result(Ok(response)))
}

fn upgrade_error<T, E>(result: Result<T, E>) -> Result<T, ApplicationError>
where E: Into<ApplicationError> {
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.into())
    }
}
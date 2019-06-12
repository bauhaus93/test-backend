
use hyper::{ Request, Response, Body, StatusCode, header };
use futures::{ Future, Stream };

use crate::application::StaticResponse;
use crate::service::{ LoginService, SimpleLoginService };
use crate::dto::Login;
use super::{ PresentationError, extract_content };

pub struct LoginController {
    login_service: Box<LoginService>
}

impl LoginController {

    pub fn new() -> Result<LoginController, PresentationError> {
        let controller = LoginController {
            login_service: Box::new(SimpleLoginService::new()?)
        }; 
        Ok(controller)
    }

    pub fn signup(&self, request: Request<Body>) -> Result<Response<Body>, PresentationError> {
        let content = extract_content(request)?;
        let login_data: Login = match serde_json::from_slice(content.as_slice()) {
            Ok(data) => data,
            Err(_e) => return Ok(StaticResponse::error_400())
        };
        info!("Signin data: {}", login_data);

        Ok(Response::default())


    }  

}


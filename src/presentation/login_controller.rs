use hyper::{ Request, Response, Body };

use crate::application::StaticResponse;
use crate::service::{ ServiceError, LoginService, SimpleLoginService };
use crate::dto::Login;
use super::{ PresentationError, extract_content, create_json_response };

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
        match self.login_service.signup(login_data) {
            Ok(session) => create_json_response(&session),
            Err(ServiceError::InsufficentData) => {
                Ok(StaticResponse::error_400())
            },
            // TODO how handle login errors
            Err(e) => {
                Err(e.into())
            }
        }
    }  

}



use hyper::{ Request, Response, Body };

use crate::service::{ LoginService, SimpleLoginService };
use super::PresentationError;

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

    pub fn signup(&self, request: Request<Body>) -> Response<Body> {
        Response::default()
    }  

}

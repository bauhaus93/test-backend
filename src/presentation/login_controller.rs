use std::sync::Arc;
use hyper::{ Request, Body };
use futures::{ Future, Stream };

use crate::application::{ ResponseFuture, static_response };
use crate::service::{ ServiceError, LoginService, SimpleLoginService };
use super::{ PresentationError, create_json_response, parse_json };

pub struct LoginController {
    login_service: Arc<LoginService>
}

impl LoginController {

    pub fn new() -> Result<LoginController, PresentationError> {
        let controller = LoginController {
            login_service: Arc::new(SimpleLoginService::new()?)
        }; 
        Ok(controller)
    }

    pub fn signup(&self, request: Request<Body>) -> ResponseFuture {
        let login_service = self.login_service.clone();
        Box::new(request.into_body().concat2()
            .map_err(|e| PresentationError::from(e))
            .and_then(|body| parse_json(body.to_vec().as_slice()))
            .and_then(move |login| {
                match login_service.signup(login) {
                    Ok(session) => create_json_response(&session),
                    Err(ServiceError::InsufficentData) => Ok(static_response::error_400()),
                    Err(e) => {
                        error!("Signup error: {}", e);
                        Ok(static_response::error_500())
                    }
                }
            })
            .map_err(|e| e.into())
        )
    }

    pub fn signin(&self, request: Request<Body>) -> ResponseFuture {
        let login_service = self.login_service.clone();
        Box::new(request.into_body().concat2()
            .map_err(|e| PresentationError::from(e))
            .and_then(|body| parse_json(body.to_vec().as_slice()))
            .and_then(move |login| {
                match login_service.signin(login) {
                    Ok(session) => create_json_response(&session),
                    Err(ServiceError::InsufficentData) => Ok(static_response::error_400()),
                    Err(e) => {
                        error!("Signin error: {}", e);
                        Ok(static_response::error_500())
                    }
                }
            })
            .map_err(|e| e.into())
        )
    }

}


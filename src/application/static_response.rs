use hyper::{ Body, Response, StatusCode };

use super::{ ApplicationError, ResponseFuture, create_response_future, read_file };

pub struct StaticResponse {
    content: String,
    status_code: StatusCode,
    content_type: String,
}

impl StaticResponse {
    pub fn from_file(file_path: &str, status_code: StatusCode, content_type: &str) -> Result<StaticResponse, ApplicationError> {
        let sr = StaticResponse {
            content: read_file(file_path)?,
            status_code: status_code,
            content_type: content_type.to_owned()
        };
        Ok(sr)
    }

    pub fn create_instance_future(&self) -> ResponseFuture {
        let result = Response::builder()
                .status(self.status_code)
                .header("Content-Type", self.content_type.as_str())
                .header("Content-Length", self.content.len())
                .body(Body::from(self.content.clone()));
        match result {
            Ok(response) => create_response_future(response),
            Err(e) => {
                error!("Could not create response: {}", e);
                Self::error_500_future()
            }
        }
    }

    pub fn error_400() -> Response<Body> {
        Self::fallback_response(400, "Bad request.")
    }

    pub fn error_404() -> Response<Body> {
        Self::fallback_response(404, "Page does not exist")
    }

    pub fn error_405() -> Response<Body> {
        Self::fallback_response(405, "Invalid method")
    }

    pub fn error_500() -> Response<Body> {
        Self::fallback_response(500, "Bad stuff happened.")
    }

    pub fn error_400_future() -> ResponseFuture {
        create_response_future(Self::error_400())
    }

    pub fn error_404_future() -> ResponseFuture {
        create_response_future(Self::error_404())
    }

    pub fn error_405_future() -> ResponseFuture {
        create_response_future(Self::error_405())
    }

    pub fn error_500_future() -> ResponseFuture {
        create_response_future(Self::error_500())
    }

    fn fallback_response(error_code: u16, error_msg: &'static str) -> Response<Body> {
        let fallback = Response::builder()
            .status(error_code)
            .header("Content-Type", "text/plain")
            .header("Content-Length", error_msg.len())
            .body(Body::from(error_msg));
        let response = match fallback {
            Ok(f) => f,
            Err(e) => {
                error!("Could not create fallback response: {}", e);
                Response::default()
            }
        };
        response
    }
}
use hyper::{ Body, Response, StatusCode };
use futures::{ future };

use super::{ ApplicationError, ResponseFuture, read_file };


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

    pub fn create_instance(&self) -> ResponseFuture {
        let result = Response::builder()
            .status(self.status_code)
            .header("Content-Type", self.content_type.as_str())
            .header("Content-Length", self.content.len())
            .body(Body::from(self.content.clone()));

        Box::new(future::result(
            match result {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into())
            }
        ))
    }



    pub fn fallback_500() -> ResponseFuture {
        Self::fallback_response(500, "Bad stuff happened.")
    }
    
    pub fn fallback_404() -> ResponseFuture {
        Self::fallback_response(404, "Page does not exist.")
    }

    fn fallback_response(error_code: u16, error_msg: &'static str) -> ResponseFuture {
        let result = Response::builder()
            .status(error_code)
            .header("Content-Type", "text/plain")
            .header("Content-Length", error_msg.len())
            .body(Body::from(error_msg));

        Box::new(future::result(
            match result {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into())
            }
        ))
    }
}
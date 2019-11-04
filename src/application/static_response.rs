use hyper::{ Body, Response };

use super::{ ResponseFuture, create_response_future };

pub fn error_400() -> Response<Body> {
    fallback_response(400, "Bad request")
}

pub fn error_404() -> Response<Body> {
    fallback_response(404, "Page does not exist")
}

pub fn error_405() -> Response<Body> {
    fallback_response(405, "Invalid method")
}

pub fn error_500() -> Response<Body> {
    fallback_response(500, "Bad stuff happened")
}

pub fn error_400_future() -> ResponseFuture {
    create_response_future(error_400())
}

pub fn error_404_future() -> ResponseFuture {
    create_response_future(error_404())
}

pub fn error_405_future() -> ResponseFuture {
    create_response_future(error_405())
}

pub fn error_500_future() -> ResponseFuture {
    create_response_future(error_500())
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

use hyper::{ Body, Response,  };
use futures::{ Future, future };

use super::ApplicationError;

pub type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=ApplicationError> + Send>;

pub fn respond_404() -> ResponseFuture {
    let response = Response::builder()
        .status(404)
        .body(Body::from("Page not found"))
        .unwrap();
    Box::new(future::result(Ok(response)))
}

pub fn respond_500() -> ResponseFuture {
    let response = Response::builder()
        .status(500)
        .body(Body::from("Bad stuff happened"))
        .unwrap();
    Box::new(future::result(Ok(response)))
}
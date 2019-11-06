use hyper::{ Body, Response };
use futures::{ Future, future };

use super::ApplicationError;

pub type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=ApplicationError> + Send>;


pub fn create_response_future(response: Response<Body>) -> ResponseFuture {
    Box::new(future::result(Ok(response)))
}
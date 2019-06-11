use hyper::{ Body, Response };
use futures::Future;

use super::ApplicationError;

pub type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=ApplicationError> + Send>;

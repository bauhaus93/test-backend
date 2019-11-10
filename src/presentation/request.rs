use futures::{Future, Stream};
use hyper::{Body, Request};
use serde::{de::DeserializeOwned, Serialize};

use super::{create_json_response, parse_json, PresentationError};
use crate::application::{static_response, ResponseFuture};
use crate::service::service_error::ServiceError;

#[macro_export]
macro_rules! create_request_handler {
    ($rq:ident, $svc:expr, $mtd:ident) => {
		{
			let svc_clone = $svc.clone();
			let func = move |i| svc_clone.$mtd(i);
			crate::presentation::request::handle_request($rq, func)
		}
	};
}

pub fn handle_request<F, T, R>(request: Request<Body>, handler_function: F) -> ResponseFuture
where
    F: Fn(T) -> Result<R, ServiceError> + Send + Sync + 'static,
    T: DeserializeOwned + Send + Sync + 'static,
    R: Serialize,
{
    Box::new(
        request
            .into_body()
            .concat2()
            .map_err(|e| PresentationError::from(e))
            .and_then(|body| {
                let data_vec = body.to_vec();
                let input_result = parse_json::<T>(data_vec.as_slice())?;
                Ok(input_result)
            })
            .and_then(move |input| match handler_function(input) {
                Ok(result) => create_json_response::<R>(&result),
                Err(ServiceError::InsufficentData) => Ok(static_response::error_400()),
                Err(e) => {
                    error!("Request error: {}", e);
                    Ok(static_response::error_500())
                }
            })
            .map_err(|e| e.into()),
    )
}

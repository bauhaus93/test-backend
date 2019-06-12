use hyper::{ Response, Body, StatusCode };
use serde::Serialize;
use serde_json;

use super::PresentationError;

pub fn create_json_response<T>(result_data: &T) -> Result<Response<Body>, PresentationError>
where T: Serialize {
    let json_str = serde_json::to_string(result_data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Content-Length", json_str.len())
        .body(Body::from(json_str))?;
    Ok(response)
}

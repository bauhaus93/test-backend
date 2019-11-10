use hyper::{ Response, Body, StatusCode };
use serde::{ Serialize, de::DeserializeOwned };
use serde_json;

use super::PresentationError;

pub fn parse_json<T>(json_data: &[u8]) -> Result<T, PresentationError>
where T: DeserializeOwned {
    match serde_json::from_slice::<T>(json_data) {
        Ok(result) => Ok(result),
        Err(e) => Err(PresentationError::from(e))
    }
}

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

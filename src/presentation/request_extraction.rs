
use hyper::{ Request, Body };
use futures::{ Future, Stream };

use super::PresentationError;

pub fn extract_content(request: Request<Body>) -> Result<Vec<u8>, PresentationError> {
    let content = request.into_body()
        .concat2()
        //.from_err()
        .wait()?;
    Ok(content.to_vec())
}
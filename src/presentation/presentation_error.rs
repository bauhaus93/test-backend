use std::fmt;
use std::error::Error;

use hyper;
use serde_json;

use crate::service::ServiceError;

#[derive(Debug)]
pub enum PresentationError {
    Service(ServiceError),
    Hyper(hyper::Error),
    HyperHttp(hyper::http::Error),
    Json(serde_json::Error)
}

impl From<ServiceError> for PresentationError {
    fn from(err: ServiceError) -> Self {
        PresentationError::Service(err)
    }
}

impl From<hyper::Error> for PresentationError {
    fn from(err: hyper::Error) -> Self {
        PresentationError::Hyper(err)
    }
}

impl From<hyper::http::Error> for PresentationError {
    fn from(err: hyper::http::Error) -> Self {
        PresentationError::HyperHttp(err)
    }
}


impl From<serde_json::Error> for PresentationError {
    fn from(err: serde_json::Error) -> PresentationError {
        PresentationError::Json(err)
    }
}

impl Error for PresentationError {

    fn description(&self) -> &str {
        match *self {
            PresentationError::Service(_) => "service",
            PresentationError::Hyper(_) => "hyper",
            PresentationError::HyperHttp(_) => "hyper",
            PresentationError::Json(_) => "json",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            PresentationError::Service(ref err) => Some(err),
            PresentationError::Hyper(ref err) => Some(err),
            PresentationError::HyperHttp(ref err) => Some(err),
            PresentationError::Json(ref err) => Some(err),
        }
    }
}

impl fmt::Display for PresentationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PresentationError::Service(ref err) => write!(f, "{}/{}", self.description(), err),
            PresentationError::Hyper(ref err) => write!(f, "{}/{}", self.description(), err),
            PresentationError::HyperHttp(ref err) => write!(f, "{}/{}", self.description(), err),
            PresentationError::Json(ref err) => write!(f, "{}: {}", self.description(), err)
        }
    }
}

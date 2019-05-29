use std::fmt;
use std::error::Error;

use crate::service::ServiceError;

#[derive(Debug)]
pub enum PresentationError {
    Service(ServiceError),
}

impl From<ServiceError> for PresentationError {
    fn from(err: ServiceError) -> Self {
        PresentationError::Service(err)
    }
}

impl Error for PresentationError {

    fn description(&self) -> &str {
        match *self {
            PresentationError::Service(_) => "service",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PresentationError::Service(ref err) => Some(err),
        }
    }
}

impl fmt::Display for PresentationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PresentationError::Service(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}

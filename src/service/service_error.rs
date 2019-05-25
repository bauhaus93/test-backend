use std::fmt;
use std::error::Error;
use std::io;

use crate::persistence::DaoError;

#[derive(Debug)]
pub enum ServiceError {
    Persistence(DaoError)
}

impl From<DaoError> for ServiceError {
    fn from(err: DaoError) -> Self {
        ServiceError::Persistence(err)
    }
}


impl Error for ServiceError {

    fn description(&self) -> &str {
        match *self {
            ServiceError::Persistence(_) => "persistence",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServiceError::Persistence(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::Persistence(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}

use std::fmt;
use std::error::Error;

use crate::persistence::DaoError;
use super::login::LoginError;

#[derive(Debug)]
pub enum ServiceError {
    Persistence(DaoError),
    Login(LoginError)
}

impl From<DaoError> for ServiceError {
    fn from(err: DaoError) -> Self {
        ServiceError::Persistence(err)
    }
}

impl From<LoginError> for ServiceError {
    fn from(err: LoginError) -> Self {
        ServiceError::Login(err)
    }
}

impl Error for ServiceError {

    fn description(&self) -> &str {
        match *self {
            ServiceError::Persistence(_) => "persistence",
            ServiceError::Login(_) => "login",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServiceError::Persistence(ref err) => Some(err),
            ServiceError::Login(ref err) => Some(err),  
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::Persistence(ref err) => write!(f, "{}/{}", self.description(), err),
            ServiceError::Login(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}

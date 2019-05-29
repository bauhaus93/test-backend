use std::fmt;
use std::error::Error;

use crate::presentation::PresentationError;

#[derive(Debug)]
pub enum ApplicationError {
    Presentation(PresentationError)    
}

impl From<PresentationError> for ApplicationError {
    fn from(err: PresentationError) -> ApplicationError {
        ApplicationError::Presentation(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Presentation(_) => "presentation"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Presentation(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Presentation(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}

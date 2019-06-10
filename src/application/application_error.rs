use std::fmt;
use std::error::Error;
use std::string::FromUtf8Error;
use hyper;

use crate::presentation::PresentationError;

#[derive(Debug)]
pub enum ApplicationError {
    Presentation(PresentationError),
    Utf8(FromUtf8Error),
    Hyper(hyper::Error),
    HyperHttp(hyper::http::Error)
}

impl From<PresentationError> for ApplicationError {
    fn from(err: PresentationError) -> ApplicationError {
        ApplicationError::Presentation(err)
    }
}

impl From<FromUtf8Error> for ApplicationError {
    fn from(err: FromUtf8Error) -> ApplicationError {
        ApplicationError::Utf8(err)
    }
}

impl From<hyper::Error> for ApplicationError {
    fn from(err: hyper::Error) -> ApplicationError {
        ApplicationError::Hyper(err)
    }
}

impl From<hyper::http::Error> for ApplicationError {
    fn from(err: hyper::http::Error) -> ApplicationError {
        ApplicationError::HyperHttp(err)
    }
}



impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Presentation(_) => "presentation",
            ApplicationError::Utf8(_) => "utf8",
            ApplicationError::Hyper(_) => "hyper",
            ApplicationError::HyperHttp(_) => "hyper-http"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Presentation(ref err) => Some(err),
            ApplicationError::Utf8(ref err) => Some(err),
            ApplicationError::Hyper(ref err) => Some(err),
            ApplicationError::HyperHttp(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Presentation(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Utf8(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Hyper(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::HyperHttp(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}

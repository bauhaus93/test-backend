use std::{ fmt, io };
use std::error::Error;
use std::string::FromUtf8Error;
use hyper;
use serde_json;

use crate::presentation::PresentationError;

#[derive(Debug)]
pub enum ApplicationError {
    Presentation(PresentationError),
    InvalidResponseName(String),
    AssetNotExisting(String),
    Utf8(FromUtf8Error),
    Hyper(hyper::Error),
    HyperHttp(hyper::http::Error),
    Io(io::Error)
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


impl From<io::Error> for ApplicationError {
    fn from(err: io::Error) -> ApplicationError {
        ApplicationError::Io(err)
    }
}


impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Presentation(_) => "presentation",
            ApplicationError::InvalidResponseName(_) => "invalid response name",
            ApplicationError::AssetNotExisting(_) => "asset not existing",
            ApplicationError::Utf8(_) => "utf8",
            ApplicationError::Hyper(_) => "hyper",
            ApplicationError::HyperHttp(_) => "http",
            ApplicationError::Io(_) => "io"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Presentation(ref err) => Some(err),
            ApplicationError::InvalidResponseName(_) => None,
            ApplicationError::AssetNotExisting(_) => None,
            ApplicationError::Utf8(ref err) => Some(err),
            ApplicationError::Hyper(ref err) => Some(err),
            ApplicationError::HyperHttp(ref err) => Some(err),
            ApplicationError::Io(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Presentation(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::InvalidResponseName(ref name) => write!(f, "{}: {}", self.description(), name),
            ApplicationError::AssetNotExisting(ref asset_name) => write!(f, "{}: {}", self.description(), asset_name),
            ApplicationError::Utf8(ref err) => write!(f, "{}: {}", self.description(), err),
            ApplicationError::Hyper(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::HyperHttp(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Io(ref err) => write!(f, "{}: {}", self.description(), err)
        }
    }
}

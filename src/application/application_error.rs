use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ApplicationError {
    
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "application error")
    }
}
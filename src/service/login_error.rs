use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LoginError {
    InvalidName,
    InvalidEmail,
    InvalidPassword,
    ExistingName,
    ExistingEmail
}

impl Error for LoginError {

    fn description(&self) -> &str {
        match *self {
            LoginError::InvalidName => "invalid name",
            LoginError::InvalidEmail => "invalid email",
            LoginError::InvalidPassword => "invalid password",
            LoginError::ExistingName => "existing name",
            LoginError::ExistingEmail => "existing email"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
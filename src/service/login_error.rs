use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LoginError {
    InvalidName,
    InvalidEmail,
    WeakPassword,
    ExistingName,
    ExistingEmail,
    NeedUsername,
    IncorrectPassword
}

impl Error for LoginError {

    fn description(&self) -> &str {
        match *self {
            LoginError::InvalidName => "invalid name",
            LoginError::InvalidEmail => "invalid email",
            LoginError::WeakPassword => "weak password",
            LoginError::ExistingName => "existing name",
            LoginError::ExistingEmail => "existing email",
            LoginError::NeedUsername => "need username",
            LoginError::IncorrectPassword => "incorrect password"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

use std::fmt;
use std::error::Error;

use postgres;

#[derive(Debug)]
pub enum DaoError {
    Postgres(postgres::Error),
    MutexPoisoned
}

impl From<postgres::Error> for DaoError {
    fn from(err: postgres::Error) -> Self {
        DaoError::Postgres(err)
    }
}


impl Error for DaoError {

    fn description(&self) -> &str {
        match *self {
            DaoError::Postgres(_) => "postgres",
            DaoError::MutexPoisoned => "mutex poisoned"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DaoError::Postgres(ref err) => Some(err),
            DaoError::MutexPoisoned => None
        }
    }
}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DaoError::Postgres(ref err) => write!(f, "{}/{}", self.description(), err),
            DaoError::MutexPoisoned => write!(f, "{}", self.description())
        }
    }
}

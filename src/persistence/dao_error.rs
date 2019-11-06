use std::fmt;
use std::error::Error;

use postgres;

#[derive(Debug)]
pub enum DaoError {
    Postgres(postgres::Error),
    MutexPoisoned,
    EntryNotFound(String, String, String)
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
            DaoError::MutexPoisoned => "mutex poisoned",
            DaoError::EntryNotFound(_, _, _) => "entry not found"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            DaoError::Postgres(ref err) => Some(err),
            DaoError::MutexPoisoned => None,
            DaoError::EntryNotFound(_, _, _) => None
        }
    }
}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DaoError::Postgres(ref err) => write!(f, "{}/{}", self.description(), err),
            DaoError::MutexPoisoned => write!(f, "{}", self.description()),
            DaoError::EntryNotFound(ref table_name, ref row_name, ref row_value) =>
                write!(f, "{}: table = '{}', key = '{}', value = '{}'", self.description(), table_name, row_name, row_value)
        }
    }
}

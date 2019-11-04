
use crate::dto::Session;
use crate::persistence::DaoError;

pub trait SessionDao: Send + Sync {
   fn add_session(&self, session: Session) -> Result<(), DaoError>;
}

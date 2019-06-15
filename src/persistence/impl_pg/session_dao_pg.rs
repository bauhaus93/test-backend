use std::sync::Mutex;
use postgres::{ Connection, TlsMode };

use crate::dto::Session;
use crate::persistence::SessionDao;
use crate::persistence::DaoError;
use super::pg_params::PG_PARAMS;

pub struct SessionDaoPg {
    connection: Mutex<Connection>
}

impl SessionDaoPg {
    pub fn new() -> Result<SessionDaoPg, DaoError> {
        trace!("Connecting to db with '{}'...", PG_PARAMS);
        let connection = Connection::connect(PG_PARAMS, TlsMode::None)?;

        let dao = SessionDaoPg {
            connection: Mutex::new(connection)
        };
        Ok(dao)
    }
}

impl SessionDao for SessionDaoPg {
    fn add_session(&self, session: Session) -> Result<(), DaoError> {
        trace!("Preparing statement for adding session...");
        let guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(_poisoned) => return Err(DaoError::MutexPoisoned)
        };
        let stmt = guard.prepare("
            INSERT INTO session (id, user_id) VALUES ($1, $2)
        ")?;
        stmt.execute(&[&session.get_id(),
                       &session.get_user_id()])?;


        Ok(())
    }
  
}

use std::sync::Mutex;
use postgres::{ Connection, TlsMode };

use crate::dto::PasswordHash;
use crate::persistence::PasswordDao;
use crate::persistence::DaoError;
use super::pg_params::PG_PARAMS;

pub struct PasswordDaoPg {
    connection: Mutex<Connection>
}

impl PasswordDaoPg {
    pub fn new() -> Result<PasswordDaoPg, DaoError> {
        trace!("Connecting to db with '{}'...", PG_PARAMS);
        let connection = Connection::connect(PG_PARAMS, TlsMode::None)?;

        let dao = PasswordDaoPg {
            connection: Mutex::new(connection)
        };
        Ok(dao)
    }
}

impl PasswordDao for PasswordDaoPg {
    fn add_password_hash(&self, password_hash: PasswordHash) -> Result<PasswordHash, DaoError> {
        trace!("Preparing statement for adding password hash...");
        let guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => return Err(DaoError::MutexPoisoned)
        };
        let stmt = guard.prepare("
            INSERT INTO password (hash, salt, user_id) VALUES ($1, $2, $3)
        ")?;
        stmt.execute(&[&password_hash.get_hash(),
                       &password_hash.get_salt(),
                       &password_hash.get_user_id()])?;

        debug!("Added password hash: {}", password_hash);

        Ok(password_hash)
    }
  
}


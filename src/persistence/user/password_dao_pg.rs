use postgres::{ Connection, TlsMode };

use crate::dto::PasswordHash;
use crate::persistence::PasswordDao;
use crate::persistence::DaoError;

pub struct PasswordDaoPg {
    connection: Connection
}

impl PasswordDaoPg {
    pub fn new(connection_params: &str) -> Result<PasswordDaoPg, DaoError> {
        trace!("Connecting to db with '{}'...", connection_params);
        let connection = Connection::connect(connection_params, TlsMode::None)?;

        let dao = PasswordDaoPg {
            connection: connection
        };
        Ok(dao)
    }
}

impl PasswordDao for PasswordDaoPg {
    fn add_password_hash(&self, password_hash: PasswordHash) -> Result<PasswordHash, DaoError> {
        trace!("Preparing statement for adding password hash...");
        let stmt = self.connection.prepare("
            INSERT INTO password (hash, salt, user_id) VALUES ($1, $2, $3)
        ")?;
        stmt.execute(&[&password_hash.get_hash(),
                       &password_hash.get_salt(),
                       &password_hash.get_user_id()])?;

        debug!("Added password hash: {}", password_hash);

        Ok(password_hash)
    }
  
}


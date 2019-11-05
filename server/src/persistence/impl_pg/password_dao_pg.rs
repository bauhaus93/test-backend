use postgres::Connection;
use std::sync::Mutex;

use super::pg_params::PG_PARAMS;
use super::try_connect;
use crate::dto::PasswordHash;
use crate::persistence::DaoError;
use crate::persistence::PasswordDao;

pub struct PasswordDaoPg {
    connection: Mutex<Connection>,
}

impl PasswordDaoPg {
    pub fn new() -> Result<PasswordDaoPg, DaoError> {
        let connection = try_connect(PG_PARAMS, 3)?;

        let dao = PasswordDaoPg {
            connection: Mutex::new(connection),
        };
        Ok(dao)
    }
}

impl PasswordDao for PasswordDaoPg {
    fn add_password_hash(&self, password_hash: PasswordHash) -> Result<PasswordHash, DaoError> {
        trace!("Preparing statement for adding password hash...");
        let guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(_poisoned) => return Err(DaoError::MutexPoisoned),
        };
        let stmt = guard.prepare(
            "
            INSERT INTO password (hash, salt, user_id) VALUES ($1, $2, $3)
        ",
        )?;
        stmt.execute(&[
            &password_hash.get_hash(),
            &password_hash.get_salt(),
            &password_hash.get_user_id(),
        ])?;

        debug!("Added password hash: {}", password_hash);

        Ok(password_hash)
    }
    fn get_password_hash_by_user_id(&self, user_id: i32) -> Result<PasswordHash, DaoError> {
        trace!("Preparing statement for getting password hash by user id...");
        let guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(_poisoned) => return Err(DaoError::MutexPoisoned),
        };
        let stmt = guard.prepare(
            "
            SELECT hash, salt, user_id
            FROM password
            WHERE user_id=$1
        ",
        )?;
        let rows = stmt.query(&[&user_id])?;

        let row = rows.get(0);
        let mut pw_hash = PasswordHash::default();
        pw_hash.set_hash(row.get::<_, Vec<u8>>(0).as_slice());
        pw_hash.set_salt(row.get::<_, Vec<u8>>(1).as_slice());
        pw_hash.set_user_id(row.get(2));

        Ok(pw_hash)
    }
}

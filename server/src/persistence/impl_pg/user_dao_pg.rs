use postgres::Connection;
use std::sync::Mutex;

use super::pg_params::PG_PARAMS;
use super::try_connect;
use crate::dto::User;
use crate::persistence::DaoError;
use crate::persistence::UserDao;

pub struct UserDaoPg {
    connection: Mutex<Connection>,
}

impl UserDaoPg {
    pub fn new() -> Result<UserDaoPg, DaoError> {
        let connection = try_connect(PG_PARAMS, 3)?;

        let dao = UserDaoPg {
            connection: Mutex::new(connection),
        };
        Ok(dao)
    }
}

impl UserDao for UserDaoPg {
    fn add_user(&self, mut user: User) -> Result<User, DaoError> {
        trace!("Preparing statement for adding user...");
        let guard = self
            .connection
            .lock()
            .or_else(|_p| return Err(DaoError::MutexPoisoned))
            .unwrap();
        let stmt = guard.prepare(
            "
            INSERT INTO user_ (name, email) VALUES ($1, $2)
            RETURNING id
        ",
        )?;
        let rows = stmt.query(&[&user.get_name(), &user.get_email()])?;
        debug_assert!(rows.len() == 1);

        let row = rows.get(0);
        let id: i32 = row.get(0);
        user.set_id(id);

        debug!("Added user: {}", user);

        Ok(user)
    }

    fn delete_user_by_id(&self, user_id: i32) -> Result<(), DaoError> {
        trace!("Preparing statement for deleting user...");
        let guard = self
            .connection
            .lock()
            .or_else(|_p| return Err(DaoError::MutexPoisoned))
            .unwrap();

        let stmt = guard.prepare(
            "
            DELETE
            FROM user_
            WHERE user_.id=$1
        ",
        )?;
        stmt.execute(&[&user_id])?;
        Ok(())
    }

    fn get_user_by_name(&self, username: &str) -> Result<User, DaoError> {
        trace!("Preparing statement for getting user...");
        let guard = self
            .connection
            .lock()
            .or_else(|_p| return Err(DaoError::MutexPoisoned))
            .unwrap();
        let stmt = guard.prepare(
            "
            SELECT id, name, email
            FROM user_
            WHERE user_.name=$1
        ",
        )?;
        let rows = stmt.query(&[&username])?;

        if rows.len() == 0 {
            Err(DaoError::EntryNotFound(
                "user_".to_owned(),
                "name".to_owned(),
                username.to_owned(),
            ))
        } else {
            let row = rows.get(0);
            let mut user = User::default();
            user.set_id(row.get(0));
            user.set_name(&row.get::<_, String>(1));
            user.set_email(&row.get::<_, String>(2));

            Ok(user)
        }
    }

    fn username_exists(&self, username: &str) -> Result<bool, DaoError> {
        let guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(_poisoned) => return Err(DaoError::MutexPoisoned),
        };
        let stmt = guard.prepare(
            "
            SELECT EXISTS(SELECT 1 FROM user_ WHERE name = $1)
        ",
        )?;
        let rows = stmt.query(&[&username])?;
        let exists: bool = rows.get(0).get(0);
        Ok(exists)
    }
    fn email_exists(&self, email: &str) -> Result<bool, DaoError> {
        let guard = self
            .connection
            .lock()
            .or_else(|_p| return Err(DaoError::MutexPoisoned))
            .unwrap();

        let stmt = guard.prepare(
            "
            SELECT EXISTS(SELECT 1 FROM user_ WHERE email = $1)
        ",
        )?;
        let rows = stmt.query(&[&email])?;
        let exists: bool = rows.get(0).get(0);
        Ok(exists)
    }
}

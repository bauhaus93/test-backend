use postgres::{ Connection, TlsMode };

use crate::dto::User;
use crate::persistence::UserDao;
use crate::persistence::DaoError;

pub struct UserDaoPg {
    connection: Connection
}

impl UserDaoPg {
    pub fn new(connection_params: &str) -> Result<UserDaoPg, DaoError> {
        trace!("Connecting to db with '{}'...", connection_params);
        let connection = Connection::connect(connection_params, TlsMode::None)?;

        let dao = UserDaoPg {
            connection: connection
        };
        Ok(dao)
    }
}

impl UserDao for UserDaoPg {
    fn add_user(&self, mut user: User) -> Result<User, DaoError> {
        trace!("Preparing statement for adding user...");
        let stmt = self.connection.prepare("
            INSERT INTO user_ (name, email) VALUES ($1, $2)
            RETURNING id
        ")?;
        let rows = stmt.query(&[&user.get_name(), &user.get_email()])?;
        debug_assert!(rows.len() == 1);

        let row = rows.get(0);
        let id: i32 = row.get(0);
        user.set_id(id);

        debug!("Added user: {}", user);

        Ok(user)
    }

    fn username_exists(&self, username: &str) -> Result<bool, DaoError> {
        let stmt = self.connection.prepare("
            SELECT EXISTS(SELECT 1 FROM user_ WHERE name = $1)
        ")?;
        let rows = stmt.query(&[&username])?;
        let exists: bool = rows.get(0).get(0);
        Ok(exists)
    }
    fn email_exists(&self, email: &str) -> Result<bool, DaoError> {
       let stmt = self.connection.prepare("
            SELECT EXISTS(SELECT 1 FROM user_ WHERE email = $1)
        ")?;
        let rows = stmt.query(&[&email])?;
        let exists: bool = rows.get(0).get(0);
        Ok(exists)
    }
}


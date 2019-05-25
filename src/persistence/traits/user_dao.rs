

use crate::dto::User;
use crate::persistence::DaoError;

pub trait UserDao {
   fn add_user(&self, user: User) -> Result<User, DaoError>;
}

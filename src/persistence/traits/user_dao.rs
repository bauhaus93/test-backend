
use crate::dto::User;
use crate::persistence::DaoError;

pub trait UserDao: Send + Sync {
   fn add_user(&self, user: User) -> Result<User, DaoError>;
   fn get_user_by_name(&self, username: &str) -> Result<User, DaoError>;
   fn username_exists(&self, username: &str) -> Result<bool, DaoError>;
   fn email_exists(&self, email: &str) -> Result<bool, DaoError>;
}

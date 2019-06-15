
use crate::dto::{ PasswordHash };
use crate::persistence::DaoError;

pub trait PasswordDao: Send + Sync {
   fn add_password_hash(&self, password_hash: PasswordHash) -> Result<PasswordHash, DaoError>;
   fn get_password_hash_by_user_id(&self, user_id: i32) -> Result<PasswordHash, DaoError>;
}

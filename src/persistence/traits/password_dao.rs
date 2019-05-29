
use crate::dto::{ PasswordHash };
use crate::persistence::DaoError;

pub trait PasswordDao: Send + Sync {
   fn add_password_hash(&self, password_hash: PasswordHash) -> Result<PasswordHash, DaoError>;
}

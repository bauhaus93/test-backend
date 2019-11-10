use crate::dto::{Session, User};
use crate::service::ServiceError;

pub trait UserService: Send + Sync {
    fn get_user_by_session(&self, session: Session) -> Result<User, ServiceError>;
}

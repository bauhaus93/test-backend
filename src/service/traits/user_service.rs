
use crate::service::ServiceError;
use crate::dto::User;

pub trait UserService {
    fn create_user(user: User) -> Result<User, ServiceError>;
}

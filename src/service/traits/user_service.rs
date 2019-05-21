
use crate::service::ServiceError;
use crate::dto::UserDTO;

pub trait UserService {
    fn create_user(user: UserDTO) -> Result<(), ServiceError>;
}

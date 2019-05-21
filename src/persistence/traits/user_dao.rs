

use crate::dto::UserDTO;
use crate::persistence::DAOError;

pub trait UserDAO {
   fn add_user(user: UserDTO) -> Result<(), DAOError>;
}

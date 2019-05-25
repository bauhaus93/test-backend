
use crate::dto::{ Login, Session };
use crate::service::ServiceError;

pub trait LoginService {
    fn signup(&self, login: Login) -> Result<Session, ServiceError>;
}
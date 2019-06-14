
use crate::dto::{ Login, Session };
use crate::service::ServiceError;

pub trait LoginService: Send + Sync {
    fn signup(&self, login: Login) -> Result<Session, ServiceError>;
    fn signin(&self, login: Login) -> Result<Session, ServiceError>;
}

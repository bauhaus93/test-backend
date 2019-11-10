
use crate::dto::{Session, User};
use crate::persistence::{SessionDao, SessionDaoPg, UserDao, UserDaoPg};
use crate::service::{ServiceError, UserService};

pub struct SimpleUserService {
    user_dao: Box<dyn UserDao>,
    session_dao: Box<dyn SessionDao>,
}

impl SimpleUserService {
    pub fn new() -> Result<SimpleUserService, ServiceError> {
        let service = SimpleUserService {
            user_dao: Box::new(UserDaoPg::new()?),
            session_dao: Box::new(SessionDaoPg::new()?),
        };

        Ok(service)
    }
}

impl UserService for SimpleUserService {
    fn get_user_by_session(&self, session: Session) -> Result<User, ServiceError> {
		let user = self.user_dao.get_user_by_session(session)?;
		Ok(user)
    }
}

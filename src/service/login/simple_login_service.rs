use std::rc::Rc;
use std::cell::RefCell;

use rand::{ Rng, FromEntropy };
use rand::rngs::StdRng;

use crate::dto::{ Login, Session };
use crate::service::ServiceError;
use crate::persistence::UserDao;
use super::{ LoginError, LoginService };

pub struct SimpleLoginService {
    rng: RefCell<StdRng>,
    user_dao: Rc<UserDao>
}

impl SimpleLoginService {
    pub fn new(user_dao: Rc<UserDao>) -> Result<SimpleLoginService, ServiceError> {
        let service = SimpleLoginService {
            rng: RefCell::new(StdRng::from_entropy()),
            user_dao: user_dao
        };
        Ok(service)
    }
}

impl LoginService for SimpleLoginService {
    fn signup(&self, login: Login) -> Result<Session, ServiceError> {
        let user = login.get_user();

        if user.get_name().is_empty() {
            return Err(LoginError::InvalidName.into());
        } else if self.user_dao.username_exists(user.get_name())? {
            return Err(LoginError::ExistingName.into());
        }

        if user.get_email().is_empty() {
            return Err(LoginError::InvalidEmail.into());
        } else if self.user_dao.email_exists(user.get_email())? {
            return Err(LoginError::ExistingEmail.into());
        }

        if check_password_strength(login.get_password()) {
            return Err(LoginError::InvalidPassword.into());
        }

        Ok(Session{})
    }
}

fn check_password_strength(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    if !password.is_ascii() {   // TODO: also include non-ascii chars
        return false;
    }
    if let None = password.find(|c: char| c.is_uppercase()) {
        return false;
    }
    if let None = password.find(|c: char| c.is_digit(10)) {
        return false;
    }
    if let None = password.find(|c: char| !c.is_alphanumeric()) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::check_password_strength;

    #[test]
    fn password_strength_empty_password() {
        assert_eq!(false, check_password_strength(""));
    }
    #[test]
    fn password_strength_only_alphabetic() {
        assert_eq!(false, check_password_strength("abcdefgh"));
    }
    #[test]
    fn password_strength_only_alphabetic_lower_case() {
        assert_eq!(false, check_password_strength("abcdefgh"));
    }
    #[test]
    fn password_strength_only_alphabetic_upper_case() {
        assert_eq!(false, check_password_strength("ABCDEFGH"));
    }
    #[test]
    fn password_strength_only_alphabetic_mixed_case() {
        assert_eq!(false, check_password_strength("AbCdeFgH"));
    }
    #[test]
    fn password_strength_only_alphanumeric_mixed_case() {
        assert_eq!(false, check_password_strength("Ab7CeF3H"));
    }
    #[test]
    fn password_strength_only_digits() {
        assert_eq!(false, check_password_strength("01234567"));
    }
    #[test]
    fn password_strength_too_short() {
        assert_eq!(false, check_password_strength("Ab[9"));
    }
    #[test]
    fn password_non_ascii() {
        assert_eq!(false, check_password_strength("Ab[9ee3´"));
    }
    #[test]
    fn password_strength_valid() {
        assert_eq!(true, check_password_strength("Ab[9ee371#"));
    }
}
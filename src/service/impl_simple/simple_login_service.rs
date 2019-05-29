use std::rc::Rc;
use std::cell::RefCell;
use sha2::{ Sha512Trunc256, Digest };

use rand::{ Rng, FromEntropy };
use rand::rngs::StdRng;

use crate::dto::{ Login, Session, PasswordHash };
use crate::persistence::{ UserDao, PasswordDao, UserDaoPg, PasswordDaoPg };
use crate::service::{ ServiceError, LoginError, LoginService };

pub struct SimpleLoginService {
    rng: RefCell<StdRng>,
    user_dao: Box<UserDao>,
    password_dao: Box<PasswordDao>
}

impl SimpleLoginService {
    pub fn new() -> Result<SimpleLoginService, ServiceError> {
        let service = SimpleLoginService {
            rng: RefCell::new(StdRng::from_entropy()),
            user_dao: Box::new(UserDaoPg::new()?),
            password_dao: Box::new(PasswordDaoPg::new()?)
        };
        Ok(service)
    }

    fn create_salted_password_hash(&self, password: &str) -> PasswordHash {
        let mut salt: [u8; 16] = [0; 16];
        self.rng.borrow_mut().fill(&mut salt);

        let mut hasher = Sha512Trunc256::new();
        hasher.input(password);
        hasher.input(salt);

        let mut hash: [u8; 32] = [0; 32];
        for (byte, output) in hash.iter_mut().zip(hasher.result().iter()) {
            *byte = *output;
        }

        let mut password_hash = PasswordHash::default();
        password_hash.set_hash(hash);
        password_hash.set_salt(salt);

        password_hash
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
        // TODO: handle users without password
        let user = self.user_dao.add_user(user.clone())?;

        let mut password_hash = self.create_salted_password_hash(login.get_password());
        password_hash.set_user_id(user.get_id());

        self.password_dao.add_password_hash(password_hash)?;

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

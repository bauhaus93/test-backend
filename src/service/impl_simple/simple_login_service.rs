use base64;
use sha2::{Digest, Sha512Trunc256};
use std::cell::RefCell;
use std::sync::Mutex;

use rand::rngs::StdRng;
use rand::{FromEntropy, Rng};

use crate::dto::{Login, PasswordHash, Session};
use crate::persistence::{
    PasswordDao, PasswordDaoPg, SessionDao, SessionDaoPg, UserDao, UserDaoPg,
};
use crate::service::{LoginError, LoginService, ServiceError};

pub struct SimpleLoginService {
    rng: Mutex<RefCell<StdRng>>,
    user_dao: Box<dyn UserDao>,
    password_dao: Box<dyn PasswordDao>,
    session_dao: Box<dyn SessionDao>,
}

impl SimpleLoginService {
    pub fn new() -> Result<SimpleLoginService, ServiceError> {
        let service = SimpleLoginService {
            rng: Mutex::new(RefCell::new(StdRng::from_entropy())),
            user_dao: Box::new(UserDaoPg::new()?),
            password_dao: Box::new(PasswordDaoPg::new()?),
            session_dao: Box::new(SessionDaoPg::new()?),
        };
        Ok(service)
    }

    fn create_salted_password_hash(&self, password: &str) -> Result<PasswordHash, ServiceError> {
        let salt = self.generate_salt()?;
        let hash = calculate_hash(password, &salt);

        let mut password_hash = PasswordHash::default();
        password_hash.set_hash(&hash);
        password_hash.set_salt(&salt);

        Ok(password_hash)
    }

    fn generate_salt(&self) -> Result<[u8; 16], ServiceError> {
        let mut salt: [u8; 16] = [0; 16];

        match self.rng.lock() {
            Ok(guard) => {
                guard.borrow_mut().fill(&mut salt);
            }
            Err(_poisoned) => {
                return Err(ServiceError::MutexPoisoned);
            }
        }
        Ok(salt)
    }

    fn generate_session_id(&self) -> Result<String, ServiceError> {
        let mut session_id: [u8; 32] = [0; 32];

        match self.rng.lock() {
            Ok(guard) => {
                guard.borrow_mut().fill(&mut session_id);
            }
            Err(_poisoned) => {
                return Err(ServiceError::MutexPoisoned);
            }
        }
        Ok(base64::encode(&session_id))
    }
}

impl LoginService for SimpleLoginService {
    fn signup(&self, login: Login) -> Result<Session, ServiceError> {
        info!("Signing up: {}", login.get_user());
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

        if !is_strong_password(login.get_password()) {
            return Err(LoginError::WeakPassword.into());
        }

        let new_user = self.user_dao.add_user(user.clone())?;

        match self.create_salted_password_hash(login.get_password()) {
            Ok(mut pw_hash) => {
                pw_hash.set_user_id(new_user.get_id());
                match self.password_dao.add_password_hash(pw_hash) {
                    Ok(_) => self.signin(login),
                    Err(add_pw_err) => {
                        // user was created, but password entry could not be added
                        if let Err(del_user_err) =
                            self.user_dao.delete_user_by_id(new_user.get_id())
                        {
                            error!("Could not delete user without password: {}", del_user_err);
                        }
                        Err(add_pw_err.into())
                    }
                }
            }
            Err(hash_creation_err) => {
                // user was created, but password hash could not be created
                if let Err(del_user_err) = self.user_dao.delete_user_by_id(new_user.get_id()) {
                    error!("Could not delete user without password: {}", del_user_err);
                }
                Err(hash_creation_err.into())
            }
        }
    }

    fn signin(&self, login: Login) -> Result<Session, ServiceError> {
        info!("Signing in: {}", login.get_user());
        let user_id = match login.get_user().get_name() {
            name if name.len() > 0 => self.user_dao.get_user_by_name(name)?.get_id(),
            _ => return Err(LoginError::NeedUsername.into()),
        };

        let saved_hash = self.password_dao.get_password_hash_by_user_id(user_id)?;
        let salt = saved_hash.get_salt();
        let input_hash = calculate_hash(login.get_password(), &salt);

        if hashes_equal(saved_hash.get_hash(), &input_hash) {
            let session_id = self.generate_session_id()?;
            let mut session = Session::default();
            session.set_id(&session_id);
            session.set_user_id(user_id);
            self.session_dao.add_session(session.clone())?;
            info!(
                "New session: user = '{}', session_id = '{}'",
                login.get_user().get_name(),
                session.get_id()
            );
            session.set_user_id(0); // Don't expose internal user id
            Ok(session)
        } else {
            info!(
                "Password hashes were not equal, no session for '{}' created.",
                login.get_user().get_name()
            );
            Err(LoginError::IncorrectPassword.into())
        }
    }
}

fn hashes_equal(hash_a: &[u8], hash_b: &[u8]) -> bool {
    debug_assert!(hash_a.len() == hash_b.len());
    hash_a == hash_b
}

fn calculate_hash(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512Trunc256::new();
    hasher.input(password);
    hasher.input(salt);

    let mut hash: [u8; 32] = [0; 32];
    for (byte, output) in hash.iter_mut().zip(hasher.result().iter()) {
        *byte = *output;
    }
    hash
}

fn is_strong_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    if !password.is_ascii() {
        // TODO: also include non-ascii chars
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
    use super::is_strong_password;

    #[test]
    fn password_strength_empty_password() {
        assert_eq!(false, is_strong_password(""));
    }
    #[test]
    fn password_strength_only_alphabetic() {
        assert_eq!(false, is_strong_password("abcdefgh"));
    }
    #[test]
    fn password_strength_only_alphabetic_lower_case() {
        assert_eq!(false, is_strong_password("abcdefgh"));
    }
    #[test]
    fn password_strength_only_alphabetic_upper_case() {
        assert_eq!(false, is_strong_password("ABCDEFGH"));
    }
    #[test]
    fn password_strength_only_alphabetic_mixed_case() {
        assert_eq!(false, is_strong_password("AbCdeFgH"));
    }
    #[test]
    fn password_strength_only_alphanumeric_mixed_case() {
        assert_eq!(false, is_strong_password("Ab7CeF3H"));
    }
    #[test]
    fn password_strength_only_digits() {
        assert_eq!(false, is_strong_password("01234567"));
    }
    #[test]
    fn password_strength_too_short() {
        assert_eq!(false, is_strong_password("Ab[9"));
    }
    #[test]
    fn password_non_ascii() {
        assert_eq!(false, is_strong_password("Ab[9ee3´"));
    }
    #[test]
    fn password_strength_valid() {
        assert_eq!(true, is_strong_password("Ab[9ee371#"));
    }
}

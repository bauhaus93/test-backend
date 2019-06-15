use std::cell::RefCell;
use std::sync::Mutex;
use sha2::{ Sha512Trunc256, Digest };

use rand::{ Rng, FromEntropy };
use rand::rngs::StdRng;

use crate::dto::{ Login, Session, PasswordHash };
use crate::persistence::{ UserDao, PasswordDao, SessionDao, UserDaoPg, PasswordDaoPg, SessionDaoPg };
use crate::service::{ ServiceError, LoginError, LoginService };

pub struct SimpleLoginService {
    rng: Mutex<RefCell<StdRng>>,
    user_dao: Box<UserDao>,
    password_dao: Box<PasswordDao>,
    session_dao: Box<SessionDao>
}

impl SimpleLoginService {
    pub fn new() -> Result<SimpleLoginService, ServiceError> {
        let service = SimpleLoginService {
            rng: Mutex::new(RefCell::new(StdRng::from_entropy())),
            user_dao: Box::new(UserDaoPg::new()?),
            password_dao: Box::new(PasswordDaoPg::new()?),
            session_dao: Box::new(SessionDaoPg::new()?)
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
            },
            Err(_poisoned) => {
                return Err(ServiceError::MutexPoisoned);
            }
        }
        Ok(salt)
    }

    fn generate_session_id(&self) -> Result<[u8; 16], ServiceError> {
        let mut session_id: [u8; 16] = [0; 16];
        
        match self.rng.lock() {
            Ok(guard) => {
                guard.borrow_mut().fill(&mut session_id);
            },
            Err(_poisoned) => {
                return Err(ServiceError::MutexPoisoned);
            }
        }
        Ok(session_id)
    }
}

impl LoginService for SimpleLoginService {
    fn signup(&self, login: Login) -> Result<Session, ServiceError> {
        info!("Signing up user: {}", login.get_user());
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

        let mut password_hash = self.create_salted_password_hash(login.get_password())?;
        
        // TODO: handle users without password
        let user = self.user_dao.add_user(user.clone())?;

        password_hash.set_user_id(user.get_id());
        self.password_dao.add_password_hash(password_hash)?;

        self.signin(login)
    }

    fn signin(&self, login: Login) -> Result<Session, ServiceError> {
        info!("Signing in user: {}", login.get_user());
        let user_id = match login.get_user().get_name() {
            name if name.len() > 0 => self.user_dao.get_user_by_name(name)?.get_id(),
            _ => return Err(LoginError::NeedUsername.into())
        };

        let saved_hash = self.password_dao.get_password_hash_by_user_id(user_id)?;
        let salt = saved_hash.get_salt();
        let input_hash = calculate_hash(login.get_password(), &salt);

        if equal_hashes(saved_hash.get_hash(), &input_hash) {
            info!("Password hashes are equal, creating session for user...");
            let session_id = self.generate_session_id()?;
            let mut session = Session::default();
            session.set_id(session_id);
            session.set_user_id(user_id);
            self.session_dao.add_session(session.clone())?;
            Ok(session)
        } else {
            Err(LoginError::IncorrectPassword.into())
        }
    }
}

fn equal_hashes(hash_a: &[u8], hash_b: &[u8]) -> bool {
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

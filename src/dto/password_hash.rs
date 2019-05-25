use std::fmt;

#[derive(Debug)]
pub struct PasswordHash {
    user_id: i32,
    hash: [u8; 32],
    salt: [u8; 16]
}

impl PasswordHash {
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
    pub fn set_hash(&mut self, hash: [u8; 32]) {
        self.hash = hash;
    }
    pub fn set_salt(&mut self, salt: [u8; 16]) {
        self.salt = salt;
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn get_salt(&self) -> &[u8] {
        &self.salt
    }
} 

impl Default for PasswordHash {
    fn default() -> PasswordHash {
        PasswordHash {
            user_id: 0,
            hash: [0; 32],
            salt: [0; 16]
        }
    }
}

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PasswordHash(user_id = {}, hash = 0x{}, salt = 0x{})",
            self.user_id,
            self.hash.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
            self.salt.iter().map(|b| format!("{:02X}", b)).collect::<String>())
    }
}


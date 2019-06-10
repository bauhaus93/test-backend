use std::fmt;

use serde::{ Serialize, Deserialize };
use super::User;

#[derive(Serialize, Deserialize, Clone)]
pub struct Login {
    #[serde(default)]
    user: User,
    #[serde(default)]
    password: String
}

impl Login {
    pub fn get_user(&self) -> &User {
        &self.user
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

impl fmt::Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Login({}, Password = '{}')", self.user, self.password)
    }
}


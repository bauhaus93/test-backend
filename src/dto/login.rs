use serde::{ Serialize, Deserialize };
use super::User;

#[derive(Serialize, Deserialize, Clone)]
pub struct Login {
    user: User,
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
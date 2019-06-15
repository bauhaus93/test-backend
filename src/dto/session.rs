
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    id: String,
    user_id: i32
}

impl Session {
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_owned();
    }
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
}

impl Default for Session {
    fn default() -> Session {
        Session {
            id: String::new(),
            user_id: 0
        }
    }
}
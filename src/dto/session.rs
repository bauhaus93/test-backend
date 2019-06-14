
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    id: [u8; 16],
    user_id: i32
}

impl Session {
    pub fn set_id(&mut self, id: [u8; 16]) {
        self.id = id;
    }
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
}

impl Default for Session {
    fn default() -> Session {
        Session {
            id: [0; 16],
            user_id: 0
        }
    }
}
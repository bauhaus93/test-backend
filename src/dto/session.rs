
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    id: String
}

impl Session {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

impl Default for Session {
    fn default() -> Session {
        Session {
            id: "666".to_owned()
        }
    }
}
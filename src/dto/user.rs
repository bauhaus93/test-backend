use std::fmt;

#[derive(Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

impl User {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_email(&mut self, email: &str) {
        self.email = email.into();
    }
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            name: "".into(),
            email: "".into()
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User(id = {}, name = '{}', email = '{}')",
            self.id,
            self.name,
            self.email)
    }
}

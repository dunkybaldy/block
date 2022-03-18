#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct User {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub total_points: i32
}

impl User {
    pub fn new(id: String, firstname: String, lastname: String, total_points: i32) -> Self { Self { id, firstname, lastname, total_points } }
}

pub struct UserBuilder {
    id: String,
    firstname: String,
    lastname: String
}

impl UserBuilder {
    pub fn new(id: &str, firstname: &str, lastname: &str) -> Self { 
        Self { id: id.to_string(), firstname: firstname.to_string(), lastname: lastname.to_string() } 
    }

    pub fn id(&mut self, id: &str) {
        self.id = id.to_string();
    }
    
    pub fn firstname(&mut self, firstname: &str) {
        self.firstname = firstname.to_string();
    }
    
    pub fn lastname(&mut self, lastname: &str) {
        self.lastname = lastname.to_string();
    }

    pub fn build(self) -> User {
        User { id: self.id, firstname: self.firstname, lastname: self.lastname, total_points: 0 }
    }
}
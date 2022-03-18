use crate::structs::user::{User, UserBuilder};

pub struct UserService {
    users: Vec<User>
}

impl UserService {
    pub fn new() -> Self { Self { users: vec![] } }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.get_users().into_iter().find(|x| x.id.eq(id))
    }

    pub fn get_users(&self) -> &[User] {
        self.users.as_ref()
    }

    pub fn get_highest_points_user(&self) {
        let mut x = self.get_users().clone();
        // x.into_iter().map(|z|z).fold(t, std::cmp::max);
    }

    pub fn new_user(&mut self, id: &str, firstname: &str, lastname: &str) {
        self.users.push(UserBuilder::new(id, firstname, lastname).build());
    }
}

use serde::{Serialize,de::DeserializeOwned};

use db::Indexable;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: u32, email: impl Into<String>) -> User {
        User {
            id,
            email: email.into(),
            ..Default::default()
        }
    }
}

// pub type Users = HashMap<u32, User>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Users {
    /// user.id -> user
    users: HashMap<u32, User>,
    last_id: u32,

    /// email -> user.id
    #[serde(skip)]
    users_by_email: HashMap<String, u32>,
}

impl Indexable for Users {
    fn reindex(&mut self) {
        self.users_by_email = HashMap::new();

        for (_, user) in &self.users {
            self.users_by_email.insert(user.email.to_string(), user.id);
        }
    }
}

impl Users {
    fn next_seq_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn get(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<User> {
        let result = self.users.remove(&id);
        self.reindex();
        result
    }

    pub fn update(&mut self, id: u32, user: User) -> Option<User> {
        let result = self.users.insert(id, user);
        self.reindex();
        result
    }

    pub fn create(&mut self, user: User) -> Option<User> {
        let mut clone = user.clone();

        clone.id = self.next_seq_id();

        let result = self.users.insert(clone.id, clone);
        self.reindex();
        result
    }

    pub fn has_email(&self, email: &String) -> bool {
        self.users_by_email.contains_key(email)
    }

    pub fn get_by_email(&self, email: &String) -> Option<&User> {
        self.users_by_email
            .get(email)
            .and_then(|id| self.users.get(id))
    }
}

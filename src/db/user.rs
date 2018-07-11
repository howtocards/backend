use db::Indexable;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            email: String::new(),
            password: String::new(),
        }
    }
}

impl User {
    pub fn new<E: Into<String>>(id: u32, email: E) -> User {
        User {
            id,
            email: email.into(),
            ..Default::default()
        }
    }
}

// pub type Users = BTreeMap<u32, User>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    users: BTreeMap<u32, User>,
    last_id: u32,

    /// (email, user)
    #[serde(skip)]
    users_by_email: BTreeMap<String, u32>,
}

impl Default for Users {
    fn default() -> Users {
        Users {
            users: Default::default(),
            last_id: Default::default(),
            users_by_email: Default::default(),
        }
    }
}

impl Indexable for Users {
    fn reindex(&mut self) {
        self.users_by_email = BTreeMap::new();

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
        let mut clone = User { ..user };

        clone.id = self.next_seq_id();

        let result = self.users.insert(clone.id, clone);
        println!("result: {:?}", result);
        self.reindex();
        result
    }

    pub fn has_email(&self, email: &String) -> bool {
        self.users_by_email.contains_key(email)
    }
}

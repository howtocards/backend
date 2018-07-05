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

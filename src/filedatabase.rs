extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fmt::Debug;


trait Record: Clone {
    fn id<'a>(&'a self) -> &'a Option<String>;

    fn set_id<'a>(&'a mut self, id: String);
}

#[derive(Serialize, Deserialize)]
struct Wrapper<T: Clone> {
    id: String,
    pub record: T,
}

impl<T: Clone> Wrapper<T> {
    fn new(id: String, record: T) -> Wrapper<T> {
        Wrapper {
            id, record
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Collection<T: Clone + Record> {
    name: String,
    items: Vec<Wrapper<T>>,
    next_sequence_value: u32,
}

impl<'a, T: Clone + Record> Collection<T> {
    fn new(name: String) -> Collection<T> {
        Collection {
            name,
            items: Vec::new(),
            next_sequence_value: 1,
        }
    }

    fn upsert(&'a mut self, record: &'a T) {
        let mut found = self.items.iter()
            .find(|&item| item.record.id() == record.id());

        if let Some(found) = found {
            found.record = record.clone()
        } else {

        }
    }
}


// ---------


#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    _id: Option<String>,
    username: String,
    password: Option<String>,
    token: Option<String>,
}

impl Default for User {
    fn default() -> User {
        User {
            _id: None,
            username: "user".to_string(),
            password: None,
            token: None,
        }
    }
}

impl User {
    fn new(id: String, username: String, password: Option<String>, token: Option<String>) -> User {
        User {
            _id: Some(id.clone()),
            username: username.clone(),
            password: password.map(|password| password.clone()),
            token: token.map(|token| token.clone()),
            ..Default::default()
        }
    }

    fn create(username: String, password: Option<String>, token: Option<String>) -> User {
        User {
            username: username.clone(),
            password: password.map(|password| password.clone()),
            token: token.map(|token| token.clone()),
            ..Default::default()
        }
    }
}


impl Record for User {
    fn id<'a>(&'a self) -> &'a Option<String> {
        &self._id
    }

    fn set_id<'a>(&'a mut self, id: String) {
        self._id = Some(id);
    }
}

// impl<T> Collection<T> {
//     fn new(name: impl Into<String>) -> Collection<T> {
//         Collection {
//             name: name.into(),
//             list: Vec::new(),
//         }
//     }
// }

pub fn test() {
    let t1 = User::create("foo".to_string(), None, None);
    let t2 = User::create("foo".to_string(), Some("bar".to_string()), None);
    let t3 = User::create("foo".to_string(), None, Some("tokenda".to_string()));
    let t4 = User::create("foo".to_string(), Some("BAAAr".to_string()), Some("tokenda".to_string()));

    // println!("{:?}", serde_json::to_string(&t1).unwrap());
    // println!("{:?}", serde_json::to_string(&t2).unwrap());
    // println!("{:?}", serde_json::to_string(&t3).unwrap());
    // println!("{:?}", serde_json::to_string(&t4).unwrap());

    let mut coll: Collection<User> = Collection::new("user".to_string());

    coll.upsert(&t1);
    coll.upsert(&t2);
    coll.upsert(&t3);
    coll.upsert(&t4);

    println!("{:?}", serde_json::to_string(&coll).expect("Cannot serialize collection"));
}

use failure::Error;
use ron;
use std::collections::BTreeMap;
use std::fs::{self, File};

mod indexable;
mod token;
mod user;

use self::indexable::Indexable;
pub use self::token::Tokens;
pub use self::user::User;

#[derive(Debug, Fail)]
pub enum DatabaseError {
    #[fail(display = "can't save database. reason: {}", reason)]
    SaveError {
        reason: String,
    },

    #[fail(display = "can't load database. reason: {}", reason)]
    LoadError {
        reason: String,
    },
}

static FILE_PATH: &'static str = "/tmp/howtocards.db.ron";

fn open_db_file() -> Result<File, DatabaseError> {
    if let Ok(file) = File::open(FILE_PATH) {
        Ok(file)
    } else if let Ok(file) = File::create(FILE_PATH) {
        Ok(file)
    } else {
        Err(DatabaseError::SaveError {
            reason: String::from("cannot open file"),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub users: BTreeMap<u32, User>,
    pub tokens: Tokens,
}

impl Default for Database {
    fn default() -> Database {
        Database {
            users: Default::default(),
            tokens: Default::default(),
        }
    }
}

impl Database {
    fn new() -> Database {
        Default::default()
    }

    pub fn save(&self) -> Result<(), DatabaseError> {
        open_db_file()?;
        let stringified = ron::ser::to_string(&self).map_err(|_| DatabaseError::SaveError {
            reason: String::from("cannot serialize db"),
        })?;

        fs::write(FILE_PATH, stringified).map_err(|_| DatabaseError::SaveError {
            reason: String::from("cannot write to a file"),
        })?;

        Ok(())
    }

    pub fn load() -> Result<Database, DatabaseError> {
        use std::io::Read;

        let mut file = open_db_file()?;

        let mut buf = String::new();

        file.read_to_string(&mut buf).or_else(|_| Ok(0))?;

        if buf.len() > 1 {
            let parsed: Database = ron::de::from_str(&buf.as_ref()).or_else(|_| Ok(Database::default()))?;

            Ok(parsed)
        } else {
            Ok(Database::new())
        }
    }

    pub fn to_string(&self) -> String {
        ron::ser::to_string(&self).unwrap_or_else(|_| String::from("<FAILED>"))
    }
}

pub(crate) fn test_db() -> Result<(), Error> {
    let mut db = Database::load()?;

    println!("{:?}", db);

    if db.users.get(&0).is_some() {
        db.users.remove(&0);
    } else {
        db.users.insert(0, user::User::new(0, "Foo"));
    }

    db.tokens.insert(0, String::from("foo"));
    db.tokens.insert(0, String::from("bar"));
    db.tokens.insert(1, String::from("baz"));

    println!("{:?}", db);

    db.tokens.remove_user_tokens(0);

    println!("{:?}", db);

    db.save()?;

    Ok(())
}

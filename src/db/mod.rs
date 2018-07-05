use failure::Error;
use ron;
use std::collections::BTreeMap;
use std::fs::{self, File};

mod token;
mod user;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub users: BTreeMap<u32, user::User>,
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

impl Database {
    fn new() -> Database {
        Database {
            users: Default::default(),
        }
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
        file.read_to_string(&mut buf).map_err(|_| DatabaseError::LoadError {
            reason: String::from("cannot read file to string"),
        })?;

        if buf.len() > 1 {
            let parsed: Database = ron::de::from_str(&buf.as_ref()).map_err(|_| DatabaseError::LoadError {
                reason: String::from("invalid database file"),
            })?;

            Ok(parsed)
        } else {
            Ok(Database::new())
        }
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

    println!("{:?}", db);

    db.save()?;

    Ok(())
}

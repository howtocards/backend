use failure::Error;
use ron;
use std::collections::BTreeMap;
use std::fs::{self, File};

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct User {
    id: u32,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    users: BTreeMap<u32, User>,
}

static FILE_PATH: &'static str = "/tmp/howtocards.db.ron";

impl Database {
    fn new() -> Database {
        Database {
            users: Default::default(),
        }
    }

    fn open() -> Result<File, DatabaseError> {
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

    fn save(&self) -> Result<(), DatabaseError> {
        Database::open()?;
        let stringified = ron::ser::to_string(&self).map_err(|_| DatabaseError::SaveError {
            reason: String::from("cannot serialize db"),
        })?;

        fs::write(FILE_PATH, stringified).map_err(|_| DatabaseError::SaveError {
            reason: String::from("cannot write to a file")
        })?;

        Ok(())
    }

    fn load() -> Result<Database, DatabaseError> {
        Ok(Database::new())
    }
}

pub(crate) fn test_db() -> Result<(), Error> {
    let mut db = Database::new();

    db.save()?;

    Ok(())
}

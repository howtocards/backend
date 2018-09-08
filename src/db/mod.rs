use failure::Error;
use ron;
use std::{
    collections::BTreeMap,
    fs::{self, File},
};

mod indexable;
mod tables;
pub mod token;
mod user;

pub use self::token::Tokens;
pub use self::user::{User, Users};
use self::{indexable::Indexable, tables::Tables};
use app_state::AppState;

pub trait Database {
    fn users(&self) -> &Users;
    fn users_mut(&mut self) -> &mut Users;

    fn tokens(&self) -> &Tokens;
    fn tokens_mut(&mut self) -> &mut Tokens;
}

#[derive(Debug, Fail)]
pub enum DatabaseError {
    #[fail(display = "can't save database. reason: {}", reason)]
    SaveError { reason: String },

    #[fail(display = "can't load database. reason: {}", reason)]
    LoadError { reason: String },
}

#[derive(Debug)]
pub struct Db {
    /// When database is saved
    file_path: String,

    tables: Tables,
}

impl Default for Db {
    fn default() -> Db {
        Db {
            file_path: String::from("/tmp/howtocards.db.ron"),
            tables: Tables::default(),
        }
    }
}

impl Database for Db {
    fn users(&self) -> &Users {
        &self.tables.users
    }

    fn users_mut(&mut self) -> &mut Users {
        &mut self.tables.users
    }

    fn tokens(&self) -> &Tokens {
        &self.tables.tokens
    }

    fn tokens_mut(&mut self) -> &mut Tokens {
        &mut self.tables.tokens
    }
}

impl Indexable for Db {
    fn reindex(&mut self) {
        self.tables.reindex();
    }
}

impl Db {
    pub fn new<F: Into<String>>(file_path: F) -> Db {
        Db {
            file_path: file_path.into(),
            ..Default::default()
        }
    }

    fn open_db_file(&self) -> Result<File, DatabaseError> {
        if let Ok(file) = File::open(&self.file_path) {
            Ok(file)
        } else if let Ok(file) = File::create(&self.file_path) {
            Ok(file)
        } else {
            Err(DatabaseError::SaveError {
                reason: String::from("cannot open file"),
            })
        }
    }

    pub fn save(&self) -> Result<(), DatabaseError> {
        self.open_db_file()?;
        let stringified =
            ron::ser::to_string(&self.tables).map_err(|_| DatabaseError::SaveError {
                reason: String::from("cannot serialize db"),
            })?;

        fs::write(&self.file_path, stringified).map_err(|_| DatabaseError::SaveError {
            reason: String::from("cannot write to a file"),
        })?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), DatabaseError> {
        use std::io::Read;

        let mut file = self.open_db_file()?;

        let mut buf = String::new();

        file.read_to_string(&mut buf).or_else(|_| Ok(0))?;

        if buf.len() > 1 {
            let parsed: Tables = ron::de::from_str(&buf.as_ref()).unwrap();
            // .or_else(|_| Ok(Db::default()))?;

            self.tables = parsed;

            self.reindex();
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn to_string(&self) -> String {
        ron::ser::to_string(&self.tables).unwrap_or_else(|_| String::from("<FAILED>"))
    }
}

pub(crate) fn test_db() -> Result<(), Error> {
    let mut db = Db::new("/tmp/howtocards.db.ron");

    db.load()?;

    println!("{:?}", db);

    if db.users().get(0).is_some() {
        db.users_mut().remove(0);
    } else {
        db.users_mut().update(0, user::User::new(0, "Foo"));
    }

    db.tokens_mut().insert(0, String::from("foo"));
    db.tokens_mut().insert(0, String::from("bar"));
    db.tokens_mut().insert(1, String::from("baz"));

    println!("{:?}", db);

    db.tokens_mut().remove_user_tokens(0);

    println!("{:?}", db);

    db.save()?;

    Ok(())
}

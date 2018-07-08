use std::sync::{Arc, Mutex};

use db::Db;

pub struct AppState {
    pub db: Arc<Mutex<Db>>,
}

impl AppState {
    pub fn new(db: Arc<Mutex<Db>>) -> AppState {
        AppState {
            db,
        }
    }
}

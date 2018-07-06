use std::sync::{Arc, Mutex};

use db::Database;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

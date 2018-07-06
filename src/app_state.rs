use std::sync::{Mutex, Arc};

use db::Database;

pub struct AppState {
  pub db: Arc<Mutex<Database>>,
}

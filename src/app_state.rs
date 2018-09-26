//! Application state types

use actix::prelude::*;
use actix_web::HttpRequest;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

use db::Db;

/// Actor with connection to postgres
pub struct DbExecutor(pub PgConnection);

/// Receives database updates
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// That state passes to each request
pub struct AppState {
    /// Old database reference
    pub db: Arc<Mutex<Db>>,

    /// Postgres connection actor
    pub pg: Addr<DbExecutor>,
}

impl AppState {
    /// Make new state
    pub fn new(db: Arc<Mutex<Db>>, pg: Addr<DbExecutor>) -> AppState {
        AppState { db, pg }
    }
}

pub type Req = HttpRequest<AppState>;

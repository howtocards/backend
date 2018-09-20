use actix::prelude::*;
use actix_web::HttpRequest;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

use db::Db;

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
    pub db: Arc<Mutex<Db>>,
    pub pg: Addr<DbExecutor>,
}

impl AppState {
    pub fn new(db: Arc<Mutex<Db>>, pg: Addr<DbExecutor>) -> AppState {
        AppState { db, pg }
    }
}

pub type Req = HttpRequest<AppState>;

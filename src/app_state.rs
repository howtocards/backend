//! Application state types

use actix::prelude::*;
use actix_web::HttpRequest;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

/// Actor with connection to postgres
pub struct DbExecutor(pub PgConnection);

/// Receives database updates
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// That state passes to each request
pub struct AppState {
    /// Postgres connection actor
    pub pg: Addr<DbExecutor>,
}

impl AppState {
    /// Make new state
    pub fn new(pg: Addr<DbExecutor>) -> AppState {
        AppState { pg }
    }
}

pub type Req = HttpRequest<AppState>;

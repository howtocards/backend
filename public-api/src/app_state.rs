//! Application state types

use actix_base::prelude::*;
use actix_web::HttpRequest;
use diesel::prelude::*;

/// Actor with connection to postgres
pub struct DbExecutor {
    pub conn: PgConnection,
}

/// Receives database updates
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(conn: PgConnection) -> Self {
        DbExecutor { conn }
    }
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

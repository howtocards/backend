//! Application state types

use actix::prelude::*;
use actix_web::HttpRequest;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};

use graphql;


pub struct GraphQLExecutor {
    pub schema: Arc<graphql::Schema>,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<graphql::Schema>) -> Self {
        Self { schema }
    }
}


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
    pub gql: Addr<GraphQLExecutor>,
}

impl AppState {
    /// Make new state
    pub fn new(pg: Addr<DbExecutor>, gql: Addr<GraphQLExecutor>) -> AppState {
        AppState { pg, gql }
    }
}

pub type Req = HttpRequest<AppState>;

//! Application library
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/38739163?s=200&v=4")]
#![allow(dead_code, unused_imports)]

extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate sha2;
extern crate uuid;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate juniper;

use actix_web::middleware::identity::IdentityService;
use actix_web::{http, middleware, server, App, HttpRequest, Json, Responder};
use diesel::PgConnection;
use failure::Fail;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub mod app_state;
pub mod auth;
pub mod auth_token;
pub mod consts;
pub mod hasher;
pub mod prelude;
pub mod time;
#[macro_use]
pub mod layer;
pub mod graphql;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;

use app_state::AppState;
use prelude::*;

fn establish_connection(db_url: String) -> PgConnection {
    use diesel::prelude::*;

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn create_server(db_url: String) -> Result<(), failure::Error> {
    env_logger::init();
    use actix::{SyncArbiter, System};
    use app_state::{DbExecutor, GraphQLExecutor};

    let system = System::new("htc-server");

    let pg = SyncArbiter::start(3, move || {
        DbExecutor::new(establish_connection(db_url.clone()))
    });

    let schema = Arc::new(graphql::create_schema());
    let gql = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    let server_creator = move || {
        let state = AppState::new(pg.clone(), gql.clone());
        App::with_state(state)
            .middleware(middleware::Logger::default())
            .middleware(
                middleware::cors::Cors::build()
                // .allowed_origin("http://127.0.0.1:9000/")
                // .send_wildcard()
                .supports_credentials()
                .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_headers(vec![http::header::CONTENT_TYPE])
                .max_age(3600)
                .finish(),
            ).middleware(IdentityService::new(auth_token::TokenIdentityPolicy::new(
                "bearer".into(),
            ))).scope("/api", routes::scope)
    };

    let app = server::new(server_creator)
        .workers(2)
        .bind("127.0.0.1:9000")
        .expect("Can not bind to 127.0.0.1:9000");

    println!("Server started on http://127.0.0.1:9000");
    app.start();
    system.run();

    Ok(())
}

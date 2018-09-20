#![allow(dead_code, unused_imports)]

extern crate actix;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate ron;
extern crate rustbreak;
extern crate sha2;
extern crate uuid;
#[macro_use]
extern crate diesel;

extern crate notify_rust;

use notify_rust::Notification;

use actix_web::middleware::identity::IdentityService;
use actix_web::{http, middleware, server, App, HttpRequest, Json, Responder};
use db::Database;
use diesel::PgConnection;
use failure::Fail;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod app_state;
mod auth;
mod auth_token;
mod consts;
mod db;
mod hasher;
mod handlers;
#[macro_use]
mod layer;
mod models;
mod routes;
mod schema;

use app_state::AppState;

fn establish_connection(db_url: String) -> PgConnection {
    use diesel::prelude::*;

    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn create_server(db_url: String) -> Result<(), failure::Error> {
    use actix::{SyncArbiter, System};
    use app_state::DbExecutor;

    let system = System::new("htc-server");

    let addr = SyncArbiter::start(4, move  || DbExecutor(establish_connection(db_url.clone())));

    let mut database = db::Db::new("/tmp/howtocards.dev_db.ron");
    database.load()?;

    let database = Arc::new(Mutex::new(database));

    let server_creator = move || {
        let db = Arc::clone(&database);
        let state = AppState::new(db, addr.clone());
        let app = App::with_state(state)
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
            )
            .middleware(IdentityService::new(auth_token::TokenIdentityPolicy::new(
                "bearer".into(),
            )));
        routes::with_app(app)
    };

    let app = server::new(server_creator)
        .workers(2)
        .bind("127.0.0.1:9000")
        .expect("Can not bind to 127.0.0.1:9000");

    Notification::new()
        .summary("HowToCards")
        .body("Backend on rust start listening")
        .appname("howtocards_backend")
        .timeout(0)
        .show()
        .unwrap();

    println!("Server started on http://127.0.0.1:9000");
    app.start();
    system.run();

    Ok(())
}

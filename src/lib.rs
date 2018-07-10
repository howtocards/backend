#![allow(dead_code, unused_imports)]
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate ron;
extern crate rustbreak;
extern crate sha2;

extern crate notify_rust;

use notify_rust::Notification;

use actix_web::{http, middleware, server, App, HttpRequest, Json, Responder};
use db::Database;
use failure::Fail;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod app_state;
mod db;
mod hasher;
mod routes;

use app_state::AppState;

pub fn create_server() -> Result<(), failure::Error> {
    let mut database = db::Db::new("/tmp/howtocards.dev_db.ron");
    database.load()?;

    let database = Arc::new(Mutex::new(database));

    let server_creator = move || {
        let db = Arc::clone(&database);
        let state = AppState::new(db);
        let app = App::with_state(state).middleware(
            middleware::cors::Cors::build()
                // .allowed_origin("http://127.0.0.1:9000/")
                // .send_wildcard()
                .supports_credentials()
                .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_headers(vec![http::header::CONTENT_TYPE])
                .max_age(3600)
                .finish(),
        );
        routes::with(app)
    };

    let app = server::new(server_creator).workers(2).bind("127.0.0.1:9000").expect("Can not bind to 127.0.0.1:9000");

    Notification::new()
        .summary("HowToCards")
        .body("Backend on rust start listening")
        .appname("howtocards_backend")
        .timeout(0)
        .show()
        .unwrap();

    println!("Server started on http://127.0.0.1:9000");
    app.run();

    Ok(())
}

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

use actix_web::{http, server, App, HttpRequest, Json, Responder};
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

    let server_creator = move || routes::with(App::with_state(AppState::new(Arc::clone(&database))));

    let app = server::new(server_creator).workers(2).bind("127.0.0.1:9000").expect("Can not bind to 127.0.0.1:9000");

    println!("Server started on http://127.0.0.1:9000");
    app.run();

    Ok(())
}

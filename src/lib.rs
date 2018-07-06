#![allow(dead_code, unused_imports)]
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate ron;
extern crate rustbreak;

use actix_web::{server, App, HttpRequest, Responder};
use failure::Fail;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

mod db;
mod app_state;

use app_state::AppState;

fn index(req: HttpRequest<AppState>) -> impl Responder {
    // let count = req.state().counter.get() + 1;

    // req.state().counter.set(count);
    // format!("Request number: {}", count)
    "Ok"
}

pub fn create_server() -> Result<(), failure::Error> {
    // db::test_db().unwrap();
    let database = db::Database::load()?;
    let database = Arc::new(Mutex::new(database));

    let app = server::new(move || {
        let db = Arc::clone(&database);
        App::with_state(AppState { db }).resource("/", |r| r.f(index))
    }).workers(2)
        .bind("127.0.0.1:9000")
        .expect("Can not bind to 127.0.0.1:9000");

    println!("Server started on http://127.0.0.1:9000");
    app.run();

    Ok(())
}

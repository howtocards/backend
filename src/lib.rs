#![allow(dead_code, unused_imports)]
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate ron;
extern crate rustbreak;

use actix_web::{http, server, App, HttpRequest, Json, Responder};
use db::Database;
use failure::Fail;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod app_state;
mod db;

use app_state::AppState;

fn index(req: HttpRequest<AppState>) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    println!("before: {:?}", db.to_string());

    let count = db.tokens().len();
    let token = format!("tok{}", count);
    db.tokens_mut().insert(count, String::from(token));
    let _ = db.save();

    println!("after: {:?}", db.to_string());

    // let count = req.state().counter.get() + 1;

    // req.state().counter.set(count);
    // format!("Request number: {}", count)
    "Ok"
}

#[derive(Deserialize)]
struct NewAccount {
    email: String,
    password: String,
}

fn create_account(account: Json<NewAccount>) -> impl Responder {
    format!("Form: email: {}, password: {}", account.email, account.password)
}

pub fn create_server() -> Result<(), failure::Error> {
    // db::test_db().unwrap();
    let mut database = db::Db::new("/tmp/howtocards.dev_db.ron");
    database.load()?;
    let database = Arc::new(Mutex::new(database));

    let app = server::new(move || {
        let db = Arc::clone(&database);
        App::with_state(AppState::new(db))
            .resource("/", |r| r.f(index))
            .resource("/account", |r| r.method(http::Method::POST).with(create_account))
    }).workers(2)
        .bind("127.0.0.1:9000")
        .expect("Can not bind to 127.0.0.1:9000");

    println!("Server started on http://127.0.0.1:9000");
    app.run();

    Ok(())
}

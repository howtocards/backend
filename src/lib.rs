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
mod hash;

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

pub fn create_server_() -> Result<(), failure::Error> {
    // db::test_db().unwrap();
    let mut database = db::Db::new("/tmp/howtocards.dev_db.ron");
    database.load()?;

    let database = Arc::new(Mutex::new(database));

    let server_creator = move || {
        let db = Arc::clone(&database);

        App::with_state(AppState::new(db))
            .resource("/", |r| r.f(index))
            .resource("/account", |r| r.method(http::Method::POST).with(create_account))
    };

    let app = server::new(server_creator).workers(2).bind("127.0.0.1:9000").expect("Can not bind to 127.0.0.1:9000");

    println!("Server started on http://127.0.0.1:9000");
    app.run();

    Ok(())
}

pub fn create_server() -> Result<(), failure::Error> {
    let r = hash::hash_string("Example");

    println!("{}", r);

    Ok(())
}

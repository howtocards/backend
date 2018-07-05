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
use std::cell::Cell;
use std::collections::HashMap;

mod db;

struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> impl Responder {
    let count = req.state().counter.get() + 1;

    req.state().counter.set(count);
    format!("Request number: {}", count)
}

pub fn create_server() {
    db::test_db().unwrap();
    // let app = server::new(|| {
    //     App::with_state(AppState {
    //         counter: Cell::new(0),
    //     }).resource("/", |r| r.f(index))
    // }).workers(2)
    //     .bind("127.0.0.1:9000")
    //     .expect("Can not bind to 127.0.0.1:9000");

    // println!("Server started on http://127.0.0.1:9000");
    // app.run();
}

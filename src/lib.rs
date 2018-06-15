#![allow(dead_code)]
#![allow(unused_imports)]

extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate ron;

use actix_web::{server, App, HttpRequest, Responder};
use std::cell::Cell;

use std::collections::BTreeMap;

struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> impl Responder {
    let count = req.state().counter.get() + 1;

    req.state().counter.set(count);
    format!("Request number: {}", count)
}

#[derive(Serialize)]
enum Type {
    User(String),
    Admin { role: u32, id: String },
    Anon,
}

#[derive(Serialize)]
struct Example {
    point: (f32, i32),
    name: String,
    lst: BTreeMap<String, i32>,
    etype: Type,
    ex: Vec<Type>,
}

pub fn create_server() {
    // let app = server::new(|| App::with_state(AppState { counter: Cell::new(0) }).resource("/", |r| r.f(index)))
    //     .workers(2)
    //     .bind("127.0.0.1:9000")
    //     .expect("Can not bind to 127.0.0.1:9000");

    // println!("Server started on http://127.0.0.1:9000");
    // app.run();

    let mut map = BTreeMap::new();
    map.insert(String::from("Foo"), 1);
    map.insert(String::from("Bar"), 2);

    let ex = Example {
        point: (123456789.1233456, 1234567890),
        name: String::from("Example"),
        lst: map,
        etype: Type::Admin { role: 20003, id: String::from("98987655234") },
        ex: vec![Type::Anon, Type::User(String::from("eeee"))],
    };
    let config = ron::ser::PrettyConfig {
        separate_tuple_members: true,
        enumerate_arrays: true,
        depth_limit: 30,
        ..Default::default()
    };
    let s = ron::ser::to_string_pretty(&ex, config).unwrap();

    println!("{}", s);
}

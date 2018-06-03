extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Foo"
}

pub fn create_server() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:9000")
        .unwrap()
        .run();
}

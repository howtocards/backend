use actix::System;
use actix_http::Response;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv;
use futures::future::{ok, Future, FutureResult, IntoFuture};
use howtocards_db::schema;
use serde::{Deserialize, Serialize};
use serde_json;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct Answer<T> {
    data: T,
}

impl<T> Answer<T> {
    fn new(data: T) -> Self {
        Answer { data }
    }

    fn into_fut(self) -> FutureResult<Self, Error> {
        ok(self)
    }
}

impl<T: Serialize> Responder for Answer<T> {
    type Error = Error;
    type Future = FutureResult<Response, Self::Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(Response::build(actix_http::http::StatusCode::OK)
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&self).expect("Unable to serialize answer")))
    }
}

fn handler() -> impl Future<Item = Answer<String>, Error = Error> {
    Answer::new(String::from("Hmmmm")).into_fut()
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let listen = &std::env::var("LISTEN").unwrap_or("localhost:9002".to_string());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be specified");

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let system = System::new("howtocards-internal-api");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/test").route(web::get().to_async(handler)))
    })
    .bind(listen)?
    .start();

    println!("Running server on {}", listen);

    system.run()
}

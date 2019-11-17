use actix::System;
use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv;
use howtocards_db::schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn main() -> std::io::Result<()> {
    let listen = &std::env::var("LISTEN").unwrap_or("localhost:9002".to_string());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be specified");

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let system = System::new("howtocards-internal-api");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
    })
    .bind(listen)?
    .start();

    println!("Running server on {}", listen);

    system.run()
}

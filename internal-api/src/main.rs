use actix_rt::System;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv;

mod answer;
mod handlers;

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
            .service(
                web::resource("/preview/card/{card_id}")
                    .route(web::post().to(handlers::card_set_preview)),
            )
    })
    .bind(listen)?
    .start();

    println!("Running server on {}", listen);

    system.run()
}

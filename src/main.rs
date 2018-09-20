extern crate dotenv;
extern crate failure;
extern crate howtocards;

use std::env;

fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    howtocards::create_server(db_url)?;

    Ok(())
}

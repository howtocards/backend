extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate howtocards;

mod prelude;
use prelude::*;
use std::env;

#[derive(Fail, Debug)]
enum StartErr {
    #[fail(display = "expected DATABASE_URL env var")]
    DbExpected,

    #[fail(display = "Check .env file exists")]
    DotEnvFail,
}

fn run() -> Result<(), failure::Error> {
    dotenv::dotenv().or_err(StartErr::DotEnvFail)?;
    let db_url = env::var("DATABASE_URL").or_err(StartErr::DbExpected)?;

    howtocards::create_server(db_url)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

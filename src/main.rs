extern crate failure;
extern crate howtocards_backend;

fn main() -> Result<(), failure::Error> {
    howtocards_backend::create_server()?;
    // .bind("127.0.0.1:9000")
    // .unwrap()
    // .run();
    Ok(())
}

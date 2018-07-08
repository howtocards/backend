extern crate failure;
extern crate howtocards;

fn main() -> Result<(), failure::Error> {
    howtocards::create_server()?;
    // .bind("127.0.0.1:9000")
    // .unwrap()
    // .run();
    Ok(())
}

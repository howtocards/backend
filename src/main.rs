extern crate failure;
extern crate howtocards;

fn main() -> Result<(), failure::Error> {
    howtocards::create_server()?;

    Ok(())
}

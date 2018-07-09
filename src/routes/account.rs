use actix_web::{Json, Responder};
use hasher;

const SALT: &'static str = "SALT";

#[derive(Deserialize)]
pub struct NewAccount {
    email: String,
    password: String,
}

pub fn create_account(account: Json<NewAccount>) -> impl Responder {
    println!("Form: email: {}, password: {}", account.email, account.password);

    let hashed = hasher::hash_password(&account.password, SALT);

    println!("Hash: {}", hashed);

    "Ok"
}

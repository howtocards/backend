use actix_web::{dev::HttpResponseBuilder, HttpResponse, Json, Responder};
use hasher;

use app_state::Req;
use consts::SALT;
use db::{Database, User};

#[derive(Deserialize, Debug)]
pub struct NewAccount {
    email: String,
    password: String,
}

pub fn create((account, req): (Json<NewAccount>, Req)) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    println!("Create account: {:?}", &account);

    if db.users().has_email(&account.email) {
        HttpResponse::BadRequest()
    } else {
        let hashed_password = hasher::hash_password(&account.password, SALT);
        let new_user = User {
            email: account.email.to_string(),
            password: hashed_password,
            ..Default::default()
        };
        db.users_mut()
            .create(new_user)
            .or(Some(Default::default()))
            .and_then(|_| db.save().ok())
            .map(|_| HttpResponse::Ok())
            .unwrap_or(HttpResponse::BadRequest())
    }
}

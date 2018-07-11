use actix_web::{HttpRequest, HttpResponse, Json, Responder, dev::HttpResponseBuilder};
use hasher;

use app_state::AppState;
use db::{Database, User};

const SALT: &'static str = "SALT";

#[derive(Deserialize)]
pub struct NewAccount {
    email: String,
    password: String,
}

pub fn create((account, req): (Json<NewAccount>, HttpRequest<AppState>)) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    if db.users().has_email(&account.email) {
        HttpResponse::BadRequest()
    } else {
        let hashed_password = hasher::hash_password(&account.password, SALT);
        let new_user = User {
            email: account.email.to_string(),
            password: hashed_password,
            ..Default::default()
        };
        db.users_mut().create(new_user)
            .or(Some(Default::default()))
            .and_then(|_| db.save().ok())
            .map(|_| HttpResponse::Ok())
            .unwrap_or(HttpResponse::BadRequest())
    }
}

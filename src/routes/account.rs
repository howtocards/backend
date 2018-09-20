use super::account_create::*;
use actix::prelude::*;
use actix_web::dev::HttpResponseBuilder;
use actix_web::Error;
use actix_web::*;
use app_state::{AppState, Req};
use consts::SALT;
use db::{Database, User};
use diesel::prelude::*;
use failure::*;
use futures::prelude::*;
use hasher;

#[derive(Deserialize, Debug)]
pub struct AccountNewRequest {
    email: String,
    password: String,
}

pub fn create((account, req): (Json<AccountCreate>, Req)) -> FutureResponse<HttpResponse> {
    use schema::users::dsl::*;

    req.state()
        .pg
        .send(account.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn create_((account, req): (Json<AccountNewRequest>, Req)) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    #[cfg(debug_assertions)]
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

#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
    app.resource("/account", |r| {
        r.method(http::Method::POST).with(self::create)
    })
}

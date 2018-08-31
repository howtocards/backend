use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{http, App, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError};
use failure::Fail;

mod account;
mod account_session;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use db::{Database, Db, User};

#[derive(Fail, Debug)]
enum IndexErrorResponse {
    #[fail(display = "unknown error")]
    Unknown,
}

impl ResponseError for IndexErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().text("")
    }
}

fn index(req: Req) -> Result<String, Error> {
    let db = req.state().db.lock().map_err(|_| IndexErrorResponse::Unknown)?;
    let count = db.tokens().len();
    let token = format!("tok{}", count);
    db.tokens_mut().insert(count, String::from(token));
    let _ = db.save();

    println!("after: {:?}", db.to_string());

    // let count = req.state().counter.get() + 1;

    // req.state().counter.set(count);
    // format!("Request number: {}", count)
    Ok("Ok".to_string())
}

fn id(auth: Auth) -> Result<String, Error> {
    Ok(format!("Welcome: {}", auth.user.email))
}

fn id_opt(auth: AuthOptional) -> Result<String, Error> {
    Ok(if let Some(user) = auth.user {
        format!("Welcome: {}", user.email)
    } else {
        format!("Hi! Anon!")
    })
}

pub fn with(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.f(index))
        .resource("/id", |r| r.method(http::Method::GET).with(id))
        .resource("/id/opt", |r| r.method(http::Method::GET).with(id_opt))
        .resource("/account", |r| r.method(http::Method::POST).with(account::create))
        .resource("/account/session", |r| {
            r.method(http::Method::POST).with(account_session::create);
            r.method(http::Method::GET).with(account_session::get)
        })
}

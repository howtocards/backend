use actix_web::{http, App, HttpRequest, Responder};

mod account;
mod account_session;

use app_state::AppState;
use db::Database;

fn index(req: HttpRequest<AppState>) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    let count = db.tokens().len();
    let token = format!("tok{}", count);
    db.tokens_mut().insert(count, String::from(token));
    let _ = db.save();

    println!("after: {:?}", db.to_string());

    // let count = req.state().counter.get() + 1;

    // req.state().counter.set(count);
    // format!("Request number: {}", count)
    "Ok"
}

pub fn with(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.f(index))
        .resource("/account", |r| r.method(http::Method::POST).with(account::create))
        .resource("/account/session", |r| {
            r.method(http::Method::POST).with(account_session::create);
            r.method(http::Method::GET).with(account_session::get)
        })
}

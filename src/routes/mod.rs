use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{
    http, App, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError,
};
use failure::Fail;

mod account;
mod account_session;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use db::{Database, Db, User};

pub fn with_app(app: App<AppState>) -> App<AppState> {
    let mut app = account::with_app(app);
    app = account_session::with_app(app);

    app
}

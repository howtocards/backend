//! Handle requests and send to handlers

use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{
    http, App, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError,
};
use failure::Fail;

pub mod account;
pub mod cards;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};

/// Applies routes to app
#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
    let mut app = account::with_app(app);
    app = cards::with_app(app);

    app
}

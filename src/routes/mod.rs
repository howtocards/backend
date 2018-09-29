//! Handle requests and send to handlers

use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::*;
use failure::Fail;

pub mod account;
pub mod cards;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};


#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope.nested("/account", account::scope)
        .nested("/cards", cards::scope)
}

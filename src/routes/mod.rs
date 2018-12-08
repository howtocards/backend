//! Handle requests and send to handlers

use actix_web::*;

pub mod account;
pub mod cards;
pub mod users;
pub mod user;

use app_state::AppState;

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .nested("/account", account::scope)
        .nested("/cards", cards::scope)
        .nested("/users", users::scope)
        .nested("/user", user::scope)
}

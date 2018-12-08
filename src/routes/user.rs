//! /user

use prelude::*;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};

#[derive(Deserialize)]
pub struct UserPath {
    user_id: u32,
}

/// GET /user/{user_id}
pub fn get_user((_req, path): (Req, Path<UserPath>)) -> String {
    path.user_id.to_string()
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope.resource("/{user_id}/", |r| r.get().with(get_user))
}

//! /account

use actix::prelude::*;
use actix_web::dev::HttpResponseBuilder;
use actix_web::Error;
use actix_web::*;
use diesel::prelude::*;
use failure::*;
use futures::prelude::*;
use serde::Serialize;

use app_state::{AppState, Req};
use auth::Auth;
use consts::SALT;
use handlers::account::create::*;
use handlers::account::login::*;
use hasher;
use layer::SuccessAnswer;

/// POST /account
pub fn create((account, req): (Json<AccountCreate>, Req)) -> FutureResponse<HttpResponse> {
    use schema::users::dsl::*;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct R {
        user_id: i32,
    }

    req.state()
        .pg
        .send(account.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(answer_success!(Created, R { user_id: user.id })),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AccountInfo {
    token: String,
    id: i32,
    email: String,
}

/// POST /account/session
pub fn login((login_data, req): (Json<SessionCreate>, Req)) -> FutureResponse<HttpResponse> {
    req.state()
        .pg
        .send(login_data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(login_info) => Ok(answer_success!(
                Ok,
                AccountInfo {
                    token: login_info.0,
                    id: login_info.1.id,
                    email: login_info.1.email,
                }
            )),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

/// GET /account/session
pub fn get_session((auth, req): (Auth, Req)) -> HttpResponse {
    use actix_web::middleware::identity::RequestIdentity;

    answer_success!(
        Ok,
        AccountInfo {
            id: auth.user.id,
            email: auth.user.email.clone(),
            token: req.identity().unwrap(),
        }
    )
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .resource("/", |r| r.post().with(self::create))
        .resource("/session/", |r| {
            r.post().with(self::login);
            r.get().with(self::get_session)
        })
}

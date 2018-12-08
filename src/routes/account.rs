//! /account

use actix_web::dev::HttpResponseBuilder;
use actix_web::Error;
use serde::Serialize;

use prelude::*;
use views;

use app_state::{AppState, Req};
use auth::Auth;
use handlers::account::create::*;
use handlers::account::login::*;

/// POST /account
pub fn create((account, req): (Json<AccountCreate>, Req)) -> FutureResponse<HttpResponse> {
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
    user: views::EncodableUserPrivate,
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
                    user: login_info.1.encodable_private(),
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
            token: req.identity().unwrap(),
            user: auth.user.encodable_private(),
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

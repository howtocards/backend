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

    req.state()
        .pg
        .send(account.0)
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().into()),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

/// POST /account/session
pub fn login((login_data, req): (Json<SessionCreate>, Req)) -> FutureResponse<HttpResponse> {
    #[derive(Serialize)]
    struct R {
        token: String,
    }

    req.state()
        .pg
        .send(login_data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(session_token) => Ok(answer_success!(
                Ok,
                R {
                    token: session_token.0,
                }
            )),
            Err(err) => Ok(err.error_response()),
        }).responder()
}

/// GET /account/session
pub fn get_session((auth, req): (Auth, Req)) -> HttpResponse {
    use actix_web::middleware::identity::RequestIdentity;

    #[derive(Serialize)]
    struct R {
        id: i32,
        email: String,
        token: String,
    }

    answer_success!(
        Ok,
        R {
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

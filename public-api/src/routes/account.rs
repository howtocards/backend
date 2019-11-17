//! /account

use crate::prelude::*;
use crate::views;

use crate::app_state::AppState;
use crate::auth::Auth;
use crate::handlers::account::create::*;
use crate::handlers::account::login::*;
use crate::handlers::account::update::*;
use actix_web::State;

/// POST /account
pub fn create(
    account: Json<AccountCreate>,
    state: State<AppState>,
) -> FutureResponse<HttpResponse> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct R {
        user_id: i32,
    }

    state
        .pg
        .send(account.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(answer_success!(Created, R { user_id: user.id })),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Update {
    display_name: String,
    gravatar_email: String,
    username: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SettingsResponse {
    settings: views::UserSettings,
}

/// PUT /account/settings/
fn update(
    state: State<AppState>,
    auth: Auth,
    update: Json<Update>,
) -> FutureResponse<HttpResponse> {
    state
        .pg
        .send(AccountUpdate {
            requester_id: auth.user.id,
            display_name: update.display_name.clone(),
            gravatar_email: update.gravatar_email.clone(),
            username: update.username.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(answer_success!(
                Ok,
                SettingsResponse {
                    settings: user.encodable_settings()
                }
            )),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

pub fn settings(auth: Auth) -> FutureResponse<HttpResponse> {
    futures::future::ok(answer_success!(
        Ok,
        SettingsResponse {
            settings: auth.user.encodable_settings()
        }
    ))
    .responder()
}

/// POST /account/session
pub fn login(
    login_data: Json<SessionCreate>,
    state: State<AppState>,
) -> FutureResponse<HttpResponse> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct R {
        token: String,
        user: views::EncodableUserPrivate,
    }

    state
        .pg
        .send(login_data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(login_info) => Ok(answer_success!(
                Ok,
                R {
                    token: login_info.token,
                    user: login_info.user.encodable_private(),
                }
            )),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionInfo {
    user: views::EncodableUserPrivate,
}

/// GET /account/session
pub fn get_session(auth: Auth) -> HttpResponse {
    answer_success!(
        Ok,
        SessionInfo {
            user: auth.user.encodable_private(),
        }
    )
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .resource("/", |r| {
            r.post().with(self::create);
        })
        .resource("/settings/", |r| {
            r.get().with(self::settings);
            r.put().with(self::update);
        })
        .resource("/session/", |r| {
            r.post().with(self::login);
            r.get().with(self::get_session)
        })
}

use actix_web::{error, http, App, HttpResponse, Json, Responder};
use std::sync::MutexGuard;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use db::{token::create_token, Database, Db, User};
use hasher;
use layer::SuccessAnswer;

use consts::SALT;

#[derive(Deserialize, Debug)]
pub struct NewSession {
    email: String,
    password: String,
}

#[derive(Debug, Fail, Serialize)]
pub enum CreateSessionError {
    #[fail(display = "email_not_found")]
    EmailNotFound,

    #[fail(display = "invalid_password")]
    InvalidPassword,
}

impl_response_error_for!(CreateSessionError as BadRequest);

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

pub fn create_session(
    session_data: Json<NewSession>,
    db: &mut MutexGuard<Db>,
) -> Result<TokenResponse, CreateSessionError> {
    #[allow(unused_assignments)]
    let mut valid_password = false;
    #[allow(unused_assignments)]
    let mut user_id = 0;

    if let Some(user) = db.users().get_by_email(&session_data.email) {
        valid_password = hasher::validate_password(&user.password, SALT, &session_data.password);
        user_id = user.id;
    } else {
        return Err(CreateSessionError::EmailNotFound);
    }

    if valid_password {
        let token = create_token();
        db.tokens_mut().insert(user_id, token.clone());
        let _ = db.save();
        Ok(TokenResponse { token })
    } else {
        Err(CreateSessionError::InvalidPassword)
    }
}

pub fn create((session_data, req): (Json<NewSession>, Req)) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    create_session(session_data, &mut db)
        .map(|token| HttpResponse::Ok().json(SuccessAnswer::new(token)))
}

#[derive(Serialize)]
pub struct SessionGetResponse {
    email: String,
}

pub fn get(auth: Auth) -> Json<SessionGetResponse> {
    Json(SessionGetResponse {
        email: auth.user.email,
    })
}

pub fn with_app(app: App<AppState>) -> App<AppState> {
    app.resource("/account/session", |r| {
        r.method(http::Method::POST).with(self::create);
        r.method(http::Method::GET).with(self::get)
    })
}

use actix_web::{error, http, HttpResponse, Json, Responder};
use app_state::Req;
use db::{token::create_token, Database, Db, User};
use hasher;
use layer::{ErrorAnswer, SuccessAnswer};
use std::sync::MutexGuard;

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

impl error::ResponseError for CreateSessionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(ErrorAnswer::new(
            match *self {
                CreateSessionError::EmailNotFound => "email_not_found",
                CreateSessionError::InvalidPassword => "invalid_password",
            }.to_string(),
        ))
    }
}

pub fn create_session(
    session_data: Json<NewSession>,
    _req: &Req,
    db: &mut MutexGuard<'_, Db>,
) -> Result<String, CreateSessionError> {
    let mut valid_password = false;
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
        Ok(token)
    } else {
        Err(CreateSessionError::InvalidPassword)
    }
}

pub fn create((session_data, req): (Json<NewSession>, Req)) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    create_session(session_data, &req, &mut db).map(|token| HttpResponse::Ok().json(SuccessAnswer::new(token)))
}

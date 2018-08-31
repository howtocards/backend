use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{http, App, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError};
use failure::Fail;

mod account;
mod account_session;

use app_state::{AppState, Req};
use db::{Database, User};

fn index(req: Req) -> impl Responder {
    let mut db = req.state().db.lock().unwrap();

    let count = db.tokens().len();
    let token = format!("tok{}", count);
    db.tokens_mut().insert(count, String::from(token));
    let _ = db.save();

    println!("after: {:?}", db.to_string());

    // let count = req.state().counter.get() + 1;

    // req.state().counter.set(count);
    // format!("Request number: {}", count)
    "Ok"
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct ApiErrorResponse {
    error: String,
    ok: bool,
}

impl ApiErrorResponse {
    pub fn from_fail(fail: &impl Fail) -> Self {
        let mut list = vec![];

        for cause in Fail::iter_chain(fail) {
            let msg = cause.to_string();
            if !list.contains(&msg) {
                list.push(msg);
            }
        }

        ApiErrorResponse { ok: false, error: list.remove(0) }
    }
}

#[derive(Fail, Debug)]
pub enum AuthError {
    #[fail(display = "invalid_token")]
    InvalidToken,

    #[fail(display = "unknown_token")]
    UnknownToken,

    #[fail(display = "missing_header")]
    MissingHeader,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(&ApiErrorResponse::from_fail(self))
    }
}

#[derive(Debug)]
pub struct Auth {
    user: User,
}

impl FromRequest<AppState> for Auth {
    type Config = ();
    type Result = Result<Auth, AuthError>;

    fn from_request(req: &HttpRequest<AppState>, _cfg: &Self::Config) -> Self::Result {
        let id = req.identity().ok_or(AuthError::InvalidToken)?.to_string();
        let db = req.state().db.lock().map_err(|_| AuthError::InvalidToken)?;

        let (_, user_id) = db.tokens().find(&id).ok_or(AuthError::UnknownToken)?;
        let user = db.users().get(user_id).ok_or(AuthError::UnknownToken)?;

        Ok(Auth { user: user.clone() })
    }
}

#[derive(Debug)]
pub struct AuthOptional {
    user: Option<User>,
}

impl FromRequest<AppState> for AuthOptional {
    type Config = ();
    type Result = Result<AuthOptional, AuthError>;

    fn from_request(req: &HttpRequest<AppState>, cfg: &Self::Config) -> Self::Result {
        Ok(AuthOptional {
            user: Auth::from_request(req, cfg).ok().map(|auth| auth.user)
        })
    }
}

fn id(auth: Auth) -> Result<String, Error> {
    Ok(format!("Welcome: {}", auth.user.email))
}

fn id_opt(auth: AuthOptional) -> Result<String, Error> {
    Ok(if let Some(user) = auth.user {
        format!("Welcome: {}", user.email)
    } else {
        format!("Hi! Anon!")
    })
}

pub fn with(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.f(index))
        .resource("/id", |r| r.method(http::Method::GET).with(id))
        .resource("/id/opt", |r| r.method(http::Method::GET).with(id_opt))
        .resource("/account", |r| r.method(http::Method::POST).with(account::create))
        .resource("/account/session", |r| {
            r.method(http::Method::POST).with(account_session::create);
            r.method(http::Method::GET).with(account_session::get)
        })
}

use actix_web::error::ErrorBadRequest;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{http, App, Error, FromRequest, HttpRequest, HttpResponse, Responder, ResponseError};
use failure::Fail;

use app_state::{AppState, Req};
use db::{Database, User};

/// Describe error that shows to user
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

        ApiErrorResponse {
            ok: false,
            error: list.remove(0),
        }
    }
}

/// Describe specific error of auth
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

/// Extractor to handle only authenticated requests
/// ```rs
/// fn example(auth: Auth) -> impl Responder {
///     let user = auth.user;
/// }
/// ```
#[derive(Debug)]
pub struct Auth {
    pub user: User,
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

/// Extractor to handle optional authentication
/// ```rs
/// fn example(auth: AuthOptional) -> impl Responder {
///     if let Some(user) = auth.user {
///         println!("Hello {}", user.name);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct AuthOptional {
    pub user: Option<User>,
}

impl FromRequest<AppState> for AuthOptional {
    type Config = ();
    type Result = Result<AuthOptional, AuthError>;

    fn from_request(req: &HttpRequest<AppState>, cfg: &Self::Config) -> Self::Result {
        Ok(AuthOptional {
            user: Auth::from_request(req, cfg).ok().map(|auth| auth.user),
        })
    }
}

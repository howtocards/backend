use actix_web::{http, App, HttpRequest, HttpResponse, Responder, Error, FromRequest, ResponseError};
use actix_web::middleware::identity::RequestIdentity;
use actix_web::error::ErrorBadRequest;
use failure::Fail;

mod account;
mod account_session;

use app_state::{Req, AppState};
use db::Database;

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
    error: String
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
            error: list.remove(0)
        }
    }
}


#[derive(Fail, Debug)]
enum AuthError {
    #[fail(display = "invalid_token")]
    InvalidToken,

    #[fail(display = "missing_header")]
    MissingHeader,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(&ApiErrorResponse::from_fail(self))
    }
}

#[derive(Debug, Deserialize)]
struct Auth {
    token: String,
}

impl<S> FromRequest<S> for Auth {
    type Config = ();
    type Result = Result<Auth, AuthError>;

    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        if let Some(id) = req.identity() {
            Ok(Auth { token: id.to_string() })
        }
        else {
            Err(AuthError::InvalidToken)
        }
    }
}

fn id((req, auth): (Req, Auth)) -> Result<String, Error> {
    // access request identity
    if let Some(id) = req.identity() {
        Ok(format!("Welcome! {}", id))
    } else {
        Ok("Welcome Anonymous!".to_owned())
    }
}


pub fn with(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.f(index))
        .resource("/id", |r| r.method(http::Method::GET).with(id))
        .resource("/account", |r| r.method(http::Method::POST).with(account::create))
        .resource("/account/session", |r| {
            r.method(http::Method::POST).with(account_session::create);
            r.method(http::Method::GET).with(account_session::get)
        })
}

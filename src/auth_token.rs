//! Authentication token parsing

use actix_web::middleware::identity::{Identity, IdentityPolicy};
use actix_web::middleware::Response;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{ok as fut_ok, FutureResult};
use std::rc::Rc;
use std::str::FromStr;

use app_state::{AppState, Req};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseAuthorizationError {
    EmptyContent,
    InvalidChunksCount,
}

#[derive(Debug, PartialEq)]
pub struct Authorization {
    prefix: String,
    value: String,
}

impl FromStr for Authorization {
    type Err = ParseAuthorizationError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        if source.len() < 3 {
            Err(ParseAuthorizationError::EmptyContent)
        } else {
            let chunks: Vec<&str> = source.split(' ').collect();

            if chunks.len() != 2 {
                Err(ParseAuthorizationError::InvalidChunksCount)
            } else {
                Ok(Authorization {
                    prefix: chunks[0].to_string(),
                    value: chunks[1].to_string(),
                })
            }
        }
    }
}

pub struct TokenIdentityInner {
    prefix: String,
}

impl TokenIdentityInner {
    fn new(prefix: String) -> Self {
        TokenIdentityInner { prefix }
    }

    fn load(&self, req: &Req) -> Option<String> {
        let auth_header = req.headers().get("Authorization")?.to_str().ok()?;
        let auth: Authorization = auth_header.parse().ok()?;

        if auth.prefix.eq(&self.prefix) {
            Some(auth.value.to_string())
        } else {
            None
        }
    }
}

pub struct TokenIdentity {
    identity: Option<String>,
    // inner: Rc<TokenIdentityInner>,
}

impl Identity for TokenIdentity {
    fn identity(&self) -> Option<&str> {
        self.identity.as_ref().map(|s| s.as_ref())
    }

    fn remember(&mut self, value: String) {
        self.identity = Some(value);
    }

    fn forget(&mut self) {
        self.identity = None;
    }

    fn write(&mut self, resp: HttpResponse) -> Result<Response, Error> {
        Ok(Response::Done(resp))
    }
}

pub struct TokenIdentityPolicy(Rc<TokenIdentityInner>);

impl TokenIdentityPolicy {
    pub fn new(prefix: String) -> Self {
        TokenIdentityPolicy(Rc::new(TokenIdentityInner::new(prefix)))
    }
}

impl IdentityPolicy<AppState> for TokenIdentityPolicy {
    type Identity = TokenIdentity;
    type Future = FutureResult<TokenIdentity, Error>;

    fn from_request(&self, req: &Req) -> Self::Future {
        let identity = self.0.load(req);

        fut_ok(TokenIdentity {
            identity,
            // inner: Rc::clone(&self.0),
        })
    }
}

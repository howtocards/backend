use actix_http::{error::ResponseError, http::StatusCode, Response};
use actix_web::{Error, HttpRequest, Responder};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Answer<S, E> {
    Success { ok: bool, result: S },
    Failed { ok: bool, error: E },
    Unexpected { ok: bool, error: String },
}

impl<S, E> Answer<S, E> {
    pub fn ok(result: S) -> Self {
        Answer::Success { ok: true, result }
    }

    pub fn fail(error: E) -> Self {
        Answer::Failed { ok: false, error }
    }

    pub fn unexpected(message: String) -> Self {
        Answer::Unexpected {
            ok: false,
            error: message,
        }
    }
}

impl<T: Serialize, E: Serialize> Responder for Answer<T, E> {
    type Error = Error;
    type Future = Ready<Result<Response, Self::Error>>;

    #[inline]
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let status = match self {
            Answer::Success { .. } => StatusCode::OK,
            Answer::Failed { .. } => StatusCode::BAD_REQUEST,
            Answer::Unexpected { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ok(Response::build(status)
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&self).expect("Unable to serialize answer")))
    }
}

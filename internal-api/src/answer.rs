use actix_http::Response;
use actix_web::{Error, HttpRequest, Responder};
use futures::future::{ok, FutureResult};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer<T> {
    ok: bool,
    result: T,
}

impl<T> Answer<T> {
    pub fn new(result: T) -> Self {
        Answer { result, ok: true }
    }

    pub fn into_fut(self) -> FutureResult<Self, Error> {
        ok(self)
    }
}

impl<T: Serialize> Responder for Answer<T> {
    type Error = Error;
    type Future = FutureResult<Response, Self::Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(Response::build(actix_http::http::StatusCode::OK)
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&self).expect("Unable to serialize answer")))
    }
}

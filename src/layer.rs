use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorAnswer {
    ok: bool,
    error: String,
}

impl ErrorAnswer {
    pub fn new(error: String) -> Self {
        ErrorAnswer { ok: false, error }
    }
}

#[derive(Serialize)]
pub struct SuccessAnswer<T> {
    ok: bool,
    result: T,
}

impl<T> SuccessAnswer<T> {
    pub fn new(result: T) -> Self {
        SuccessAnswer { ok: true, result }
    }
}

macro_rules! impl_response_error_for {
    ($struct:ident as $response_status:ident) => {
        use actix_web;
        use layer as lay;
        impl actix_web::error::ResponseError for $struct {
            fn error_response(&self) -> actix_web::HttpResponse {
                actix_web::HttpResponse::$response_status()
                    .json(lay::ErrorAnswer::new(format!("{}", self)))
            }
        }
    };
}

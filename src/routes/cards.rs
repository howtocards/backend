use actix_web::{dev::HttpResponseBuilder, http, App, HttpResponse, Json, Responder};

use app_state::{AppState, Req};
use db::{Database, User};



#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
  app
}

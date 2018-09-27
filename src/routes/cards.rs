use actix_web::{dev::HttpResponseBuilder, http, App, HttpResponse, Json, Responder};

use app_state::{AppState, Req};



#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
  app
}

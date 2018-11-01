use actix::prelude::*;
use actix_web::*;
use failure::*;
use futures::*;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use models::*;

type FutRes = FutureResponse<HttpResponse>;

type UserPath = Path<(u32,)>;

// pub fn info((auth, req, path): (AuthOptional, Req, UserPath)) -> FutRes {
//     use handlers::users::get_use::*;

//     #[derive(Serialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct R {

//     }
// }

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope.resource("/user/{user_id}", |r| {
        // r.get().with(self::info);
    })
}

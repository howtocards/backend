use actix_web::*;
use failure::*;
use futures::*;

use app_state::{AppState, Req};
use auth::AuthOptional;

use handlers::users::get_user::*;
use models::User;
use views::{EncodableUserPrivate, EncodableUserPublic};

type FutRes = FutureResponse<HttpResponse>;

type UserPath = Path<(u32,)>;

#[derive(Fail, Debug)]
enum GetUserInfoError {
    #[fail(display = "user_not_found")]
    NotFound,
}

pub fn info((auth, req, path): (AuthOptional, Req, UserPath)) -> FutRes {
    #[derive(Serialize)]
    #[serde(untagged)]
    enum R {
        Public { user: EncodableUserPublic },
        Private { user: EncodableUserPrivate },
    }

    impl R {
        #[inline]
        fn answer(auth: AuthOptional, user: User) -> R {
            if auth
                .user
                .map(|auth_user| auth_user.id == user.id)
                .unwrap_or(false)
            {
                R::Private {
                    user: user.encodable_private(),
                }
            } else {
                R::Public {
                    user: user.encodable_public(),
                }
            }
        }
    }

    req.state()
        .pg
        .send(GetUser {
            user_id: path.0 as i32,
        }).from_err()
        .and_then(|res| match res {
            Some(user) => Ok(answer_success!(Ok, R::answer(auth, user))),
            None => Ok(answer_error!(
                NotFound,
                GetUserInfoError::NotFound.to_string()
            )),
        }).responder()
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope.resource("/{user_id}/", |r| {
        r.get().with(self::info);
    })
}

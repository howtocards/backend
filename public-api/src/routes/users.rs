use crate::prelude::*;

use crate::app_state::AppState;
use crate::auth::AuthOptional;

use crate::models::{Card, User};
use crate::views::{EncodableUserPrivate, EncodableUserPublic};

type FutRes = FutureResponse<HttpResponse>;

#[derive(Deserialize)]
pub struct UserPath {
    username: String,
}

#[derive(Fail, Debug)]
enum GetUserInfoError {
    #[fail(display = "user_not_found")]
    NotFound,
}
/// GET /users/{username}/
pub fn info(auth: AuthOptional, path: Path<UserPath>, state: State<AppState>) -> FutRes {
    use crate::handlers::users::get_user::*;

    #[derive(Serialize)]
    struct R {
        user: UserView,
    }

    #[derive(Serialize)]
    #[serde(untagged)]
    enum UserView {
        Public(EncodableUserPublic),
        Private(EncodableUserPrivate),
    }

    impl UserView {
        #[inline]
        fn answer(auth: AuthOptional, user: User) -> Self {
            if auth
                .user
                .map(|auth_user| auth_user.id == user.id)
                .unwrap_or(false)
            {
                UserView::Private(user.encodable_private())
            } else {
                UserView::Public(user.encodable_public())
            }
        }
    }

    state
        .pg
        .send(GetUser {
            username: path.username.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Some(user) => Ok(answer_success!(
                Ok,
                R {
                    user: UserView::answer(auth, user),
                }
            )),
            None => Ok(answer_error!(
                NotFound,
                GetUserInfoError::NotFound.to_string()
            )),
        })
        .responder()
}

#[derive(Deserialize)]
pub struct CardsQuery {
    count: Option<u32>,
}

/// GET /users/{username}/cards/useful/
pub fn useful(
    _auth: AuthOptional,
    path: Path<UserPath>,
    state: State<AppState>,
    query: Query<CardsQuery>,
) -> FutRes {
    use crate::handlers::users::useful_cards::*;
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct R {
        cards: Vec<Card>,
    }

    state
        .pg
        .send(GetUsefulCardsForUser {
            username: path.username.clone(),
            count: query.count.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Some(cards) => Ok(answer_success!(Ok, R { cards })),
            None => Ok(answer_success!(Ok, R { cards: vec![] })),
        })
        .responder()
}

/// GET /users/{username}/cards/authors/
/// Get cards by user
pub fn authors(
    _auth: AuthOptional,
    path: Path<UserPath>,
    state: State<AppState>,
    query: Query<CardsQuery>,
) -> FutRes {
    use crate::handlers::users::cards_by_author::*;
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct R {
        cards: Vec<Card>,
    }

    state
        .pg
        .send(GetCardsByAuthor {
            author_username: path.username.clone(),
            count: query.count.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Some(cards) => Ok(answer_success!(Ok, R { cards })),
            None => Ok(answer_success!(Ok, R { cards: vec![] })),
        })
        .responder()
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .resource("/{username}/", |r| {
            r.get().with(self::info);
        })
        .resource("/{username}/cards/useful/", |r| {
            r.get().with(self::useful);
        })
        .resource("/{username}/cards/authors/", |r| {
            r.get().with(self::authors)
        })
}

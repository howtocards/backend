//! /cards
use crate::prelude::*;
use serde_json::Value;

use crate::app_state::AppState;
use crate::auth::{Auth, AuthOptional};
use crate::models::*;
use actix_web::State;

type FutRes = FutureResponse<HttpResponse>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardCreateBody {
    content: Value,
    title: String,
}

/// POST /cards
pub fn create(card_form: Json<CardCreateBody>, auth: Auth, state: State<AppState>) -> FutRes {
    state
        .pg
        .send(CardNew {
            author_id: auth.user.id,
            content: card_form.0.content,
            title: card_form.0.title,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(created) => Ok(answer_success!(Ok, created)),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

/// GET /cards
pub fn list(auth: AuthOptional, state: State<AppState>) -> FutRes {
    use crate::handlers::cards::list::*;

    #[derive(Serialize)]
    pub struct R(Vec<Card>);

    state
        .pg
        .send(CardsListFetch {
            requester_id: auth.user.map(|user| user.id),
        })
        .from_err()
        .and_then(|res| match res {
            Some(list) => Ok(answer_success!(Ok, R(list))),
            None => Ok(answer_success!(Ok, R(vec![]))),
        })
        .responder()
}

#[derive(Deserialize)]
pub struct CardPath {
    card_id: u32,
}

/// GET /cards/{card_id}
pub fn get(auth: AuthOptional, path: Path<CardPath>, state: State<AppState>) -> FutRes {
    use crate::handlers::cards::get::*;

    #[derive(Serialize)]
    pub struct R {
        card: Card,
    }

    state
        .pg
        .send(CardFetch {
            card_id: path.card_id,
            requester_id: auth.user.map(|user| user.id),
        })
        .from_err()
        .and_then(|res| match res {
            Some(card) => Ok(answer_success!(Ok, R { card })),
            None => Ok(answer_error!(NotFound, "id_not_found".to_string())),
        })
        .responder()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardEditBody {
    content: Option<Value>,
    title: Option<String>,
}

/// PUT /cards/{card_id}
pub fn edit(
    auth: Auth,
    path: Path<CardPath>,
    edit_form: Json<CardEditBody>,
    state: State<AppState>,
) -> FutRes {
    use crate::handlers::cards::edit::*;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct R {
        card: Card,
    }

    state
        .pg
        .send(CardEdit {
            card_id: path.card_id,
            requester_id: auth.user.id,
            title: edit_form.0.title,
            content: edit_form.0.content,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(card) => Ok(answer_success!(Ok, R { card })),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

/// DELETE /cards/{card_id}
pub fn delete(auth: Auth, path: Path<CardPath>, state: State<AppState>) -> FutRes {
    use crate::handlers::cards::delete::*;

    #[derive(Serialize)]
    pub struct R {
        card: Card,
    }

    state
        .pg
        .send(CardDelete {
            requester_id: auth.user.id,
            card_id: path.card_id,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(card) => Ok(answer_success!(Accepted, R { card })),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCardUseful {
    is_useful: bool,
}

/// POST /cards/{card_id}/useful/
pub fn toggle_useful(
    auth: Auth,
    path: Path<CardPath>,
    body: Json<SetCardUseful>,
    state: State<AppState>,
) -> FutRes {
    use crate::handlers::cards::toggle_useful_mark::*;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct R {
        card: Card,
    }

    state
        .pg
        .send(ToggleUsefulMark {
            requester_id: auth.user.id,
            card_id: path.card_id as i32,
            set_is_useful: body.is_useful,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(card) => Ok(answer_success!(Ok, R { card })),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .resource("/{card_id}/", |r| {
            r.get().with(self::get);
            r.put().with(self::edit);
            r.delete().with(self::delete);
        })
        .resource("/{card_id}/useful/", |r| {
            r.post().with(self::toggle_useful);
        })
        .resource("/", |r| {
            r.post().with(self::create);
            r.get().with(self::list);
        })
}

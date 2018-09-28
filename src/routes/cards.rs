use actix::prelude::*;
use actix_web::*;
use failure::*;
use futures::*;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use models::*;

type FutRes = FutureResponse<HttpResponse>;

#[derive(Deserialize)]
pub struct CardCreateBody {
    content: String,
    title: String,
}

/// POST /cards
pub fn create((card_form, auth, req): (Json<CardCreateBody>, Auth, Req)) -> FutRes {
    use handlers::cards::create::*;

    #[derive(Serialize)]
    struct R {
        card: Card,
    }

    req.state()
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

#[derive(Deserialize)]
pub struct CardEditBody {
    content: Option<String>,
    title: Option<String>,
}

pub fn edit((edit_form, auth, req, path): (Json<CardEditBody>, Auth, Req, Path<(u32,)>)) -> FutRes {
    use handlers::cards::edit::*;

    #[derive(Serialize)]
    pub struct R {
        card: Card,
    }

    req.state()
        .pg
        .send(CardEdit {
            card_id: path.0,
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

/// GET /cards
pub fn list((_auth, req): (AuthOptional, Req)) -> FutRes {
    use handlers::cards::list::*;

    #[derive(Serialize)]
    pub struct R(Vec<Card>);

    req.state()
        .pg
        .send(CardsListFetch)
        .from_err()
        .and_then(|res| match res {
            Some(list) => Ok(answer_success!(Ok, R(list))),
            None => Ok(answer_success!(Ok, R(vec![]))),
        })
        .responder()
}

type CardPath = Path<(u32,)>;

/// GET /cards/{card_id}
pub fn get((_auth, req, path): (AuthOptional, Req, CardPath)) -> FutRes {
    use handlers::cards::get::*;

    #[derive(Serialize)]
    pub struct R {
        card: Card,
    }

    req.state()
        .pg
        .send(CardFetch { id: path.0 })
        .from_err()
        .and_then(|res| match res {
            Some(card) => Ok(answer_success!(Ok, R { card })),
            None => Ok(answer_error!(NotFound, "id_not_found".to_string())),
        })
        .responder()
}

/// DELETE /cards/{card_id}
pub fn delete((auth, req, path): (Auth, Req, CardPath)) -> FutRes {
    use handlers::cards::delete::*;

    #[derive(Serialize)]
    pub struct R {
        card: Card,
    }

    req.state()
        .pg
        .send(CardDelete {
            requester_id: auth.user.id,
            card_id: path.0,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(card) => Ok(answer_success!(Accepted, R { card })),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
    app.resource("/cards/{card_id}", |r| {
        r.method(http::Method::GET).with(self::get);
        r.method(http::Method::PUT).with(self::edit);
        r.method(http::Method::DELETE).with(self::delete);
    }).resource("/cards", |r| {
        r.method(http::Method::POST).with(self::create);
        r.method(http::Method::GET).with(self::list)
    })
}

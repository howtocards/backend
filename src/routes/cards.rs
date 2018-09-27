use actix::prelude::*;
use actix_web::*;
use failure::*;
use futures::*;

use app_state::{AppState, Req};
use auth::Auth;
use handlers::cards::create::*;
use models::*;

#[derive(Deserialize)]
pub struct CardCreateBody {
    content: String,
    title: String,
}

/// POST /cards
pub fn create(
    (card_form, auth, req): (Json<CardCreateBody>, Auth, Req),
) -> FutureResponse<HttpResponse> {
    use schema::cards::dsl::*;

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

#[inline]
pub fn with_app(app: App<AppState>) -> App<AppState> {
    app.resource("/cards", |r| {
        r.method(http::Method::POST).with(self::create)
    })
}

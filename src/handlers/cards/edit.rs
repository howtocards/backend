//! Edit existing card

use actix::prelude::*;
use actix_web::*;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;

use app_state::{DbExecutor, Req};
use layer::ErrorAnswer;
use models::*;
use prelude::*;
use time;

#[derive(Fail, Debug)]
pub enum CardEditError {
    /// When card with id not found
    #[fail(display = "card_not_found")]
    NotFound,

    /// When diesel returns any error
    #[fail(display = "incorrect_form")]
    IncorrectForm,

    /// When user is not author of the card
    #[fail(display = "no_acess")]
    NoRights,
}

impl ResponseError for CardEditError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CardEditError::NotFound => HttpResponse::NotFound(),
            CardEditError::IncorrectForm => HttpResponse::BadRequest(),
            CardEditError::NoRights => HttpResponse::Forbidden(),
        }.json(ErrorAnswer::new(format!("{}", self)))
    }
}

pub struct CardEdit {
    pub card_id: u32,
    /// User id who requested edit of card
    pub requester_id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Message for CardEdit {
    type Result = Result<Card, CardEditError>;
}

impl Handler<CardEdit> for DbExecutor {
    type Result = Result<Card, CardEditError>;

    fn handle(&mut self, msg: CardEdit, _ctx: &mut Self::Context) -> Self::Result {
        use diesel::RunQueryDsl;
        use schema::cards::dsl::*;

        let target = cards.filter(id.eq(msg.card_id as i32));

        let found = target
            .get_result::<Card>(&self.0)
            .or_err(CardEditError::NotFound)?;

        if found.author_id != msg.requester_id {
            Err(CardEditError::NoRights)?;
        }

        let update = diesel::update(target).set((
            updated_at.eq(Some(time::now())),
            title.eq(msg.title.unwrap_or(found.title)),
            content.eq(msg.content.unwrap_or(found.content)),
        ));

        Ok(update
            .get_result::<Card>(&self.0)
            .or_err(CardEditError::IncorrectForm)?)
    }
}

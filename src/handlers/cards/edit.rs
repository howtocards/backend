//! Edit existing card

use actix_base::prelude::*;
use actix_web::*;
use diesel;

use crate::app_state::DbExecutor;
use crate::layer::ErrorAnswer;
use crate::models::*;
use crate::prelude::*;
use crate::sanitize::sanitize;

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
        }
        .json(ErrorAnswer::new(format!("{}", self)))
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
        let found = Card::find_by_id(&self.conn, msg.card_id as i32, msg.requester_id)
            .ok_or(CardEditError::NotFound)?;

        if found.author_id != msg.requester_id {
            Err(CardEditError::NoRights)?;
        }

        let new_content = msg
            .content
            .map(|html| sanitize(&html))
            .unwrap_or(found.content);

        Card::update(
            &self.conn,
            msg.card_id as i32,
            msg.requester_id,
            msg.title.unwrap_or(found.title),
            new_content,
        )
        .ok_or(CardEditError::IncorrectForm)
    }
}

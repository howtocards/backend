//! Create card

use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

#[derive(Fail, Debug)]
pub enum CardCreateError {
    /// When received empty `title` or/and `content`
    #[fail(display = "empty_title_or_content")]
    EmptyTitleContent,

    /// When diesel returns any error
    #[fail(display = "incorrect_form")]
    IncorrectForm,
}

impl_response_error_for!(CardCreateError as BadRequest);

impl Message for CardNew {
    type Result = Result<Card, CardCreateError>;
}

impl Handler<CardNew> for DbExecutor {
    type Result = Result<Card, CardCreateError>;

    fn handle(&mut self, msg: CardNew, _: &mut Self::Context) -> Self::Result {
        if msg.title.len() > 2 {
            let card = CardNew {
                content: msg.content,
                ..msg
            };

            Card::create(&self.conn, card, msg.author_id).ok_or(CardCreateError::IncorrectForm)
        } else {
            Err(CardCreateError::EmptyTitleContent)
        }
    }
}

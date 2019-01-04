//! Delete existing card

use actix_base::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

#[derive(Fail, Debug)]
pub enum CardDeleteError {
    /// When card already deleted
    #[fail(display = "card_not_found")]
    NotFound,

    /// When user is not author of the card
    #[fail(display = "no_access")]
    NoRights,
}

impl_response_error_for!(CardDeleteError as Forbidden);

pub struct CardDelete {
    pub card_id: u32,
    /// User id who requested delete
    pub requester_id: i32,
}

/// Message returns deleted card
impl Message for CardDelete {
    type Result = Result<Card, CardDeleteError>;
}

impl Handler<CardDelete> for DbExecutor {
    type Result = Result<Card, CardDeleteError>;

    fn handle(&mut self, msg: CardDelete, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::cards::dsl::*;
        use diesel::RunQueryDsl;

        let target = cards
            .filter(id.eq(msg.card_id as i32))
            .filter(author_id.eq(msg.requester_id));

        let found = target
            .select(select_card(msg.requester_id))
            .get_result::<Card>(&self.conn)
            .or_err(CardDeleteError::NotFound)?;

        diesel::delete(target)
            .execute(&self.conn)
            .or_err(CardDeleteError::NoRights)?;

        Ok(found)
    }
}

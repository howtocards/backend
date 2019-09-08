//! Delete existing card

use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

#[derive(Fail, Debug)]
pub enum CardDeleteError {
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
        Card::delete(&self.conn, msg.card_id as i32, msg.requester_id)
            .ok_or(CardDeleteError::NoRights)
    }
}

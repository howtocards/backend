//! Get single card

use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

/// Fetch single card
///
/// Should be sended to DbExecutor
pub struct CardFetch {
    pub card_id: u32,
    pub requester_id: Option<i32>,
}

impl Message for CardFetch {
    type Result = Option<Card>;
}

impl Handler<CardFetch> for DbExecutor {
    type Result = Option<Card>;

    fn handle(&mut self, msg: CardFetch, _ctx: &mut Self::Context) -> Self::Result {
        Card::find_by_id(
            &self.conn,
            msg.card_id as i32,
            msg.requester_id.unwrap_or(-1),
        )
    }
}

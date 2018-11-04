//! Get single card

use actix::prelude::*;

use app_state::DbExecutor;
use models::*;

/// Fetch single card
///
/// Should be sended to DbExecutor
pub struct CardFetch {
    pub card_id: u32,
}

impl Message for CardFetch {
    type Result = Option<Card>;
}

impl Handler<CardFetch> for DbExecutor {
    type Result = Option<Card>;

    fn handle(&mut self, msg: CardFetch, _ctx: &mut Self::Context) -> Self::Result {
        Card::find_by_id(&self.conn, msg.card_id as i32)
    }
}

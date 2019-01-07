//! Cards list

use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

/// Fetch all cards
///
/// TODO: need params
/// Should be sended to DbExecutor
pub struct CardsListFetch {
    pub requester_id: Option<i32>,
}

impl Message for CardsListFetch {
    type Result = Option<Vec<Card>>;
}

impl Handler<CardsListFetch> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: CardsListFetch, _ctx: &mut Self::Context) -> Self::Result {
        Some(Card::get_latest_cards(
            &self.conn,
            msg.requester_id.unwrap_or(-1),
        ))
    }
}

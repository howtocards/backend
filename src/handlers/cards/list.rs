//! Cards list

use actix_base::prelude::*;
use diesel::prelude::*;

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
        use crate::schema::cards::dsl::*;

        cards
            .select(select_card(msg.requester_id.unwrap_or(-1)))
            .get_results::<Card>(&self.conn)
            .ok()
            .map(|list| list.into_iter().rev().collect())
    }
}

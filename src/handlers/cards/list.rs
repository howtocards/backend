//! Cards list

use actix::prelude::*;
use diesel::prelude::*;

use app_state::DbExecutor;
use models::*;

/// Fetch all cards
///
/// TODO: need params
/// Should be sended to DbExecutor
pub struct CardsListFetch;

impl Message for CardsListFetch {
    type Result = Option<Vec<Card>>;
}

impl Handler<CardsListFetch> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, _msg: CardsListFetch, _ctx: &mut Self::Context) -> Self::Result {
        use schema::cards::dsl::*;

        cards
            .get_results::<Card>(&self.conn)
            .ok()
            .map(|list| list.into_iter().rev().collect())
    }
}

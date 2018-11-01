//! Get single card

use actix::prelude::*;
use diesel::prelude::*;

use app_state::DbExecutor;
use models::*;

/// Fetch single card
///
/// Should be sended to DbExecutor
pub struct CardFetch {
    pub id: u32,
}

impl Message for CardFetch {
    type Result = Option<Card>;
}

impl Handler<CardFetch> for DbExecutor {
    type Result = Option<Card>;

    fn handle(&mut self, msg: CardFetch, _ctx: &mut Self::Context) -> Self::Result {
        use schema::cards::dsl::*;

        cards
            .find(msg.id as i32)
            .get_result::<Card>(&self.conn)
            .ok()
    }
}

//! Get single card

use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use app_state::{DbExecutor, Req};
use layer::ErrorAnswer;
use models::*;
use prelude::*;

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

        cards.find(msg.id as i32).get_result::<Card>(&self.0).ok()
    }
}

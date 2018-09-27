//! Cards list

use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use app_state::{DbExecutor, Req};
use layer::ErrorAnswer;
use models::*;
use prelude::*;

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

        cards.get_results::<Card>(&self.0).ok()
    }
}

use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetCardsByAuthor {
    pub author_id: i32,
}

impl Message for GetCardsByAuthor {
    type Result = Option<Vec<Card>>;
}

impl Handler<GetCardsByAuthor> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: GetCardsByAuthor, _ctx: &mut Self::Context) -> Self::Result {
        Some(Card::find_all_by_author(&self.conn, msg.author_id))
    }
}

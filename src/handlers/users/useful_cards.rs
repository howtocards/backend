use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetUsefulCardsForUser {
    pub user_id: i32,
}

impl Message for GetUsefulCardsForUser {
    type Result = Option<Vec<Card>>;
}

impl Handler<GetUsefulCardsForUser> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: GetUsefulCardsForUser, _ctx: &mut Self::Context) -> Self::Result {
        Some(Card::get_useful_for_user(&self.conn, msg.user_id))
    }
}

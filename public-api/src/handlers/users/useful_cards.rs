use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetUsefulCardsForUser {
    pub username: String,
}

impl Message for GetUsefulCardsForUser {
    type Result = Option<Vec<Card>>;
}

impl Handler<GetUsefulCardsForUser> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: GetUsefulCardsForUser, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(user) = User::find_by_username(&self.conn, msg.username) {
            Some(Card::get_useful_for_user(&self.conn, user.id))
        } else {
            None
        }
    }
}

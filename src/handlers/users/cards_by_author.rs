use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetCardsByAuthor {
    pub author_username: String,
}

impl Message for GetCardsByAuthor {
    type Result = Option<Vec<Card>>;
}

impl Handler<GetCardsByAuthor> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: GetCardsByAuthor, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(user) = User::find_by_username(&self.conn, msg.author_username) {
            Some(Card::find_all_by_author(&self.conn, user.id))
        } else {
            None
        }
    }
}

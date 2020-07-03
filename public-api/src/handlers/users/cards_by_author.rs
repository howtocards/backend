use actix_base::prelude::*;
use std::cmp;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetCardsByAuthor {
    pub author_username: String,
    pub count: Option<u32>,
}

const DEFAULT_COUNT: u32 = 20;
const MAX_COUNT: u32 = 50;

impl Message for GetCardsByAuthor {
    type Result = Option<Vec<Card>>;
}

impl Handler<GetCardsByAuthor> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: GetCardsByAuthor, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(user) = User::find_by_username(&self.conn, msg.author_username) {
            Some(Card::find_all_by_author(
                &self.conn,
                user.id,
                cmp::max(msg.count.unwrap_or(DEFAULT_COUNT), MAX_COUNT),
            ))
        } else {
            None
        }
    }
}

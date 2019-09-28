use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

pub struct GetUser {
    pub username: String,
}

impl Message for GetUser {
    type Result = Option<User>;
}

impl Handler<GetUser> for DbExecutor {
    type Result = Option<User>;

    fn handle(&mut self, msg: GetUser, _ctx: &mut Self::Context) -> Self::Result {
        User::find_by_username(&self.conn, msg.username)
    }
}

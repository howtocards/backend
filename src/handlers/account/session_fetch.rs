///! Fetch user from session token
use actix_base::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;

/// Fetch user account from session token
///
/// Should be sended to DbExecutor
#[derive(Debug)]
pub struct AccountSessionFetch {
    pub token: String,
}

impl Message for AccountSessionFetch {
    type Result = Option<User>;
}

impl Handler<AccountSessionFetch> for DbExecutor {
    type Result = Option<User>;

    fn handle(&mut self, msg: AccountSessionFetch, _ctx: &mut Self::Context) -> Self::Result {
        User::find_by_token(&self.conn, msg.token)
    }
}

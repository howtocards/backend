///! Fetch user from session token
use actix_base::prelude::*;
use diesel::prelude::*;

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
        use crate::schema::tokens::dsl::*;
        use crate::schema::users::dsl::*;

        let found_token: Token = tokens.filter(token.eq(msg.token)).first(&self.conn).ok()?;

        let found_user: User = users
            .find(found_token.user_id)
            .get_result(&self.conn)
            .ok()?;

        Some(found_user)
    }
}

///! Fetch user from session token
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use app_state::{DbExecutor, Req};
use consts;
use hasher;
use layer::ErrorAnswer;
use models::*;
use prelude::*;

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
        use schema::tokens::dsl::*;
        use schema::users::dsl::*;

        let found_token: Token = tokens.filter(token.eq(msg.token)).first(&self.conn).ok()?;

        let found_user: User = users
            .find(found_token.user_id)
            .get_result(&self.conn)
            .ok()?;

        Some(found_user)
    }
}

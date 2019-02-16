//! Create account
use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::consts;
use crate::hasher;
use crate::models::*;
use crate::prelude::*;

#[derive(Fail, Debug)]
pub enum AccountCreateError {
    /// When email already exists in db
    #[fail(display = "email_already_exists")]
    EmailExists,
}

impl_response_error_for!(AccountCreateError as BadRequest);

/// Account create message
///
/// Should be sended to DbExecutor
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountCreate {
    pub email: String,
    pub password: String,
}

impl Message for AccountCreate {
    type Result = Result<User, AccountCreateError>;
}

impl Handler<AccountCreate> for DbExecutor {
    type Result = Result<User, AccountCreateError>;

    fn handle(&mut self, msg: AccountCreate, _: &mut Self::Context) -> Self::Result {
        let new_account = UserNew {
            email: msg.email,
            password: hasher::hash_password(&msg.password, consts::SALT),
        };

        User::create(&self.conn, new_account).ok_or(AccountCreateError::EmailExists)
    }
}

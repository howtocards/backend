//! Update account settings
use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

#[derive(Fail, Debug)]
pub enum AccountUpdateError {
    #[fail(display = "nothing_to_update")]
    NothingToUpdate,

    #[fail(display = "failed_to_update")]
    Failed,
}

impl_response_error_for!(AccountUpdateError as BadRequest);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    pub requester_id: i32,
    pub display_name: String,
    pub gravatar_email: String,
}

impl Message for AccountUpdate {
    type Result = Result<User, AccountUpdateError>;
}

impl Handler<AccountUpdate> for DbExecutor {
    type Result = Result<User, AccountUpdateError>;

    fn handle(&mut self, msg: AccountUpdate, _: &mut Self::Context) -> Self::Result {
        User::update(
            &self.conn,
            msg.requester_id,
            msg.display_name,
            msg.gravatar_email,
        )
        .ok_or(AccountUpdateError::Failed)
    }
}
//! Update account settings
use actix_base::{Handler, Message};
use failure::Fail;

use crate::app_state::DbExecutor;
use crate::models::user::{UpdateError, User};

#[derive(Fail, Debug)]
pub enum AccountUpdateError {
    #[fail(display = "nothing_to_update")]
    NothingToUpdate,

    #[fail(display = "failed_to_update")]
    Failed,

    #[fail(display = "username_empty")]
    UsernameEmpty,

    #[fail(display = "username_incorrect")]
    UsernameIncorrect,

    #[fail(display = "username_taken")]
    UsernameTaken,
}

impl_response_error_for!(AccountUpdateError as BadRequest);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    pub requester_id: i32,
    pub display_name: String,
    pub gravatar_email: String,
    pub username: String,
}

impl Message for AccountUpdate {
    type Result = Result<User, AccountUpdateError>;
}

impl Handler<AccountUpdate> for DbExecutor {
    type Result = Result<User, AccountUpdateError>;

    fn handle(&mut self, msg: AccountUpdate, _: &mut Self::Context) -> Self::Result {
        if msg.username.trim().is_empty() {
            Err(AccountUpdateError::UsernameEmpty)
        } else if !crate::validators::check_username(msg.username.as_str()) {
            Err(AccountUpdateError::UsernameIncorrect)
        } else {
            User::update(
                &self.conn,
                msg.requester_id,
                msg.display_name,
                msg.gravatar_email,
                msg.username,
            )
            .map_err(|error| match error {
                UpdateError::UsernameTaken => AccountUpdateError::UsernameTaken,
                _ => AccountUpdateError::Failed,
            })
        }
    }
}

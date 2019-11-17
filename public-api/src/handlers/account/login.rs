//! Session create
use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::consts;
use crate::hasher;
use crate::models::*;
use crate::prelude::*;

#[derive(Debug, Fail, Serialize)]
pub enum SessionCreateError {
    /// When user id not found in db
    #[fail(display = "user_not_found")]
    UserNotFound,

    /// When happened something terrible like session string already exists
    #[fail(display = "cant_create_session")]
    TokenInsertFail,
}

impl_response_error_for!(SessionCreateError as BadRequest);

/// Pass data to router
pub struct SessionToken {
    pub token: String,
    pub user: User,
}

/// Session create message
///
/// Should be sended to DbExecutor
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionCreate {
    pub email: String,
    pub password: String,
}

impl Message for SessionCreate {
    type Result = Result<SessionToken, SessionCreateError>;
}

impl Handler<SessionCreate> for DbExecutor {
    type Result = Result<SessionToken, SessionCreateError>;

    fn handle(&mut self, msg: SessionCreate, _: &mut Self::Context) -> Self::Result {
        let credentials = Credentials {
            email: msg.email,
            password: hasher::hash_password(&msg.password, consts::SALT),
        };

        let user = User::find_by_credentials(&self.conn, credentials)
            .ok_or(SessionCreateError::UserNotFound)?;

        let token = Token::create(&self.conn, user.id)
            .ok_or(SessionCreateError::TokenInsertFail)?
            .token;

        Ok(SessionToken { token, user })
    }
}

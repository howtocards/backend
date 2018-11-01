//! Session create

use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use app_state::{DbExecutor, Req};
use consts;
use hasher;
use layer::ErrorAnswer;
use models::*;
use prelude::*;

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
pub struct SessionToken(pub String, pub User);

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
        use diesel::RunQueryDsl;
        use schema::tokens::dsl::*;
        use schema::users::dsl::*;
        use schema::{tokens, users};

        let new_account = UserNew {
            email: msg.email,
            password: hasher::hash_password(&msg.password, consts::SALT),
        };

        let user = users::table
            .filter(users::email.eq(new_account.email.clone()))
            .filter(users::password.eq(new_account.password.clone()))
            .get_result::<User>(&self.conn)
            .or_err(SessionCreateError::UserNotFound)?;

        let token_string = format!("{}-{}", Uuid::new_v4(), Uuid::new_v4());

        let new_token = Token {
            token: token_string,
            user_id: user.id,
        };

        diesel::insert_into(tokens::table)
            .values(&new_token)
            .execute(&self.conn)
            .or_err(SessionCreateError::TokenInsertFail)?;

        Ok(SessionToken(new_token.token, user))
    }
}

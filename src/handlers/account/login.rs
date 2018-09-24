use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use models::*;
use hasher;
use app_state::{DbExecutor, Req};
use layer::ErrorAnswer;
use consts;

#[derive(Debug, Fail, Serialize)]
pub enum SessionCreateError {
  #[fail(display = "user_not_found")]
  UserNotFound,
}

impl_response_error_for!(SessionCreateError as BadRequest);

pub struct SessionToken(pub String);

#[derive(Deserialize, Debug)]
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
            email: &msg.email,
            password: &hasher::hash_password(&msg.password, consts::SALT),
        };

        let user = users::table
          .filter(users::email.eq(&msg.email))
          .filter(users::password.eq(new_account.password.clone()))
          .get_result::<User>(&self.0)
          .map_err(|_| SessionCreateError::UserNotFound)?;

        println!("{:#?}", user);

        Ok(SessionToken("example".to_string()))
    }
}

use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use app_state::DbExecutor;
use app_state::Req;
use consts;
use hasher;
use models::*;

pub struct SessionToken(pub String);

#[derive(Deserialize, Debug)]
pub struct SessionCreate {
    pub email: String,
    pub password: String,
}

impl Message for SessionCreate {
    type Result = Result<SessionToken, Error>;
}

impl Handler<SessionCreate> for DbExecutor {
    type Result = Result<SessionToken, Error>;

    fn handle(&mut self, msg: SessionCreate, _: &mut Self::Context) -> Self::Result {
        use diesel::RunQueryDsl;
        use schema::tokens::dsl::*;
        use schema::users::dsl::*;
        use schema::{tokens, users};

        Ok(SessionToken("example".to_string()))
    }
}

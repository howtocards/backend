use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use app_state::DbExecutor;
use app_state::Req;
use models::*;

#[derive(Deserialize, Debug)]
pub struct AccountCreate {
    pub email: String,
    pub password: String,
}

impl Message for AccountCreate {
    type Result = Result<User, Error>;
}

impl Handler<AccountCreate> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: AccountCreate, _: &mut Self::Context) -> Self::Result {
        use schema::users;
        use schema::users::dsl::*;
        // use diesel::prelude::*;
        use diesel::RunQueryDsl;

        let new_account = UserNew {
            email: &msg.email,
            password: &msg.password,
        };

        Ok(diesel::insert_into(users)
            .values(&new_account)
            .get_result::<User>(&self.0)
            .map_err(|_| error::ErrorInternalServerError("Failed"))?)
    }
}

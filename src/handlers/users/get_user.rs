use actix::prelude::*;
use diesel::prelude::*;

use app_state::DbExecutor;
use models::*;

pub struct GetUser {
    user_id: i32,
}

impl Message for GetUser {
    type Result = Option<User>;
}

impl Handler<GetUser> for DbExecutor {
    type Result = Option<User>;

    fn handle(&mut self, msg: GetUser, _ctx: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;

        users
            .filter(id.eq(msg.user_id))
            .get_result::<User>(&self.conn)
            .ok()
    }
}

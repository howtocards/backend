//! Check is card useful for user

use actix_base::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use crate::app_state::DbExecutor;
use crate::prelude::*;

/// May fail when SetMarkCardUseful sended to DbExecutor
#[derive(Fail, Debug)]
pub enum IsCardUsefulError {
    #[fail(display = "user_not_found")]
    UserNotFound,

    #[fail(display = "card_not_found")]
    CardNotFound,
}

impl_response_error_for!(IsCardUsefulError as BadRequest);

/// Check is card useful for user
pub struct IsCardUseful {
    pub card_id: i32,
    pub user_id: i32,
}

impl Message for IsCardUseful {
    type Result = Result<bool, IsCardUsefulError>;
}

impl Handler<IsCardUseful> for DbExecutor {
    type Result = Result<bool, IsCardUsefulError>;

    fn handle(&mut self, msg: IsCardUseful, _ctx: &mut Self::Context) -> Self::Result {
        use diesel::RunQueryDsl;

        use crate::schema::useful_marks::dsl::*;
        use diesel::dsl::count;

        let count = useful_marks
            .filter(card_id.eq(msg.card_id))
            .filter(user_id.eq(msg.user_id))
            .select(count(card_id))
            .first(&self.conn)
            .unwrap_or(0);

        Ok(count > 0)
    }
}

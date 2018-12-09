//! Mark card as useful

use actix_base::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;
use crate::time;

/// May fail when SetMarkCardUseful sended to DbExecutor
#[derive(Fail, Debug)]
pub enum MarkCardUsefulError {
    #[fail(display = "user_not_found")]
    UserNotFound,

    #[fail(display = "card_not_found")]
    CardNotFound,
}

impl_response_error_for!(MarkCardUsefulError as BadRequest);

/// Mark/Unmark card useful
pub struct SetMarkCardUseful {
    pub card_id: i32,
    pub requester_id: i32,
    pub set_is_useful: bool,
}

impl Message for SetMarkCardUseful {
    type Result = Result<Card, MarkCardUsefulError>;
}

impl Handler<SetMarkCardUseful> for DbExecutor {
    type Result = Result<Card, MarkCardUsefulError>;

    fn handle(&mut self, msg: SetMarkCardUseful, _ctx: &mut Self::Context) -> Self::Result {
        use diesel::RunQueryDsl;

        // TODO refactor to much less requests to db

        let card = {
            // Check if cards exists
            use crate::schema::cards::dsl::*;

            cards
                .filter(id.eq(msg.card_id))
                .get_result::<Card>(&self.conn)
                .or_err(MarkCardUsefulError::CardNotFound)?
        };
        {
            // Check if user exists
            use crate::schema::users::dsl::*;

            users
                .filter(id.eq(msg.requester_id))
                .get_result::<User>(&self.conn)
                .or_err(MarkCardUsefulError::UserNotFound)?;
        };

        {
            // Delete previous mark, if exists
            use crate::schema::useful_marks::dsl::*;

            let filter = useful_marks
                .filter(card_id.eq(msg.card_id))
                .filter(user_id.eq(msg.requester_id));
            let _ = diesel::delete(filter).execute(&self.conn);
        }

        if msg.set_is_useful {
            use crate::schema::useful_marks::dsl::*;

            let mark = UsefulMark {
                card_id: msg.card_id,
                user_id: msg.requester_id,
                created_at: time::now(),
            };

            let _ = diesel::insert_into(useful_marks)
                .values(&mark)
                .execute(&self.conn);
        }

        let useful_count: i64 = {
            use crate::schema::useful_marks::dsl::*;
            use diesel::dsl::count;

            useful_marks
                .filter(card_id.eq(msg.card_id))
                .filter(user_id.eq(msg.requester_id))
                .select(count(card_id))
                .first(&self.conn)
                .unwrap_or(0)
        };

        let new_card = {
            // Update card with `updated_at` and `useful_for`
            use crate::schema::cards::dsl::*;
            let filter = cards.filter(id.eq(msg.card_id));

            diesel::update(filter)
                .set((
                    updated_at.eq(Some(time::now())),
                    useful_for.eq(useful_count),
                ))
                .get_result(&self.conn)
                .unwrap_or(card)
        };

        Ok(new_card)
    }
}

//! Mark card as useful

use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

/// May fail when SetMarkCardUseful sended to DbExecutor
#[derive(Fail, Debug)]
pub enum ToggleUsefulMarkError {
    #[fail(display = "user_not_found")]
    UserNotFound,

    #[fail(display = "card_not_found")]
    CardNotFound,
}

impl_response_error_for!(ToggleUsefulMarkError as BadRequest);

/// Mark/Unmark card useful
pub struct ToggleUsefulMark {
    pub card_id: i32,
    pub requester_id: i32,
    pub set_is_useful: bool,
}

impl Message for ToggleUsefulMark {
    type Result = Result<Card, ToggleUsefulMarkError>;
}

impl Handler<ToggleUsefulMark> for DbExecutor {
    type Result = Result<Card, ToggleUsefulMarkError>;

    fn handle(&mut self, msg: ToggleUsefulMark, _ctx: &mut Self::Context) -> Self::Result {
        // TODO refactor to much less requests to db

        // Check if cards exists
        let card = Card::find_by_id(&self.conn, msg.card_id, msg.requester_id)
            .ok_or(ToggleUsefulMarkError::CardNotFound)?;

        // Check if user exists
        let _user = User::find_by_id(&self.conn, msg.requester_id)
            .ok_or(ToggleUsefulMarkError::UserNotFound)?;

        if msg.set_is_useful {
            UsefulMark::create(&self.conn, msg.card_id, msg.requester_id);
        } else {
            UsefulMark::delete(&self.conn, msg.card_id, msg.requester_id);
        }

        let useful_count: i64 = UsefulMark::count_for_card(&self.conn, msg.card_id);

        let new_card =
            Card::update_useful_for(&self.conn, msg.card_id, useful_count, msg.requester_id)
                .unwrap_or(card);

        Ok(new_card)
    }
}

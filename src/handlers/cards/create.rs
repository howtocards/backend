//! Create card

use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;

use app_state::{DbExecutor, Req};
use layer::ErrorAnswer;
use models::*;
use prelude::*;

#[derive(Fail, Debug)]
pub enum CardCreateError {
    /// When received empty `title` or/and `content`
    #[fail(display = "empty_title_or_content")]
    EmptyTitleContent,

    /// When diesel returns any error
    #[fail(display = "incorrect_form")]
    IncorrectForm,
}

impl_response_error_for!(CardCreateError as BadRequest);

impl Message for CardNew {
    type Result = Result<Card, CardCreateError>;
}

impl Handler<CardNew> for DbExecutor {
    type Result = Result<Card, CardCreateError>;

    fn handle(&mut self, msg: CardNew, _: &mut Self::Context) -> Self::Result {
        use diesel::RunQueryDsl;
        use schema::cards::dsl::*;

        Ok(diesel::insert_into(cards)
            .values(&msg)
            .get_result::<Card>(&self.0)
            .or_err(CardCreateError::IncorrectForm)?)
    }
}

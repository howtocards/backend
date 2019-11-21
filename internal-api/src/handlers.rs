use crate::answer::Answer;
use actix_web::{web, Error};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use futures::future::Future;

use howtocards_db::schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn card_set_preview(
    poll: web::Data<PgPoll>,
) -> impl Future<Item = Answer<String>, Error = Error> {
    Answer::new(String::from("Hmmmm")).into_fut()
}

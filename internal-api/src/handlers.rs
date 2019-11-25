use crate::answer::Answer;
use actix_web::{web, Error};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use howtocards_db::schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn card_set_preview(
    poll: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<Answer<String>, Error> {
    Ok(Answer::new(String::from("Hmmmm")))
}

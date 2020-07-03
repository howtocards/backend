use crate::answer::Answer;
use actix_web::{web, Error as AWError};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use howtocards_db::{diesel, schema};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPreviewBody {
    snapshot: String,
    screenshot: String,
}

#[derive(Debug, Deserialize)]
pub struct CardPath {
    card_id: u32,
}

#[derive(Debug, Serialize)]
pub struct Example {
    id: u32,
}

#[derive(Debug, Serialize)]
pub enum SetPreviewError {
    DatabaseFailure,
}

impl<T, E> Into<Answer<T, E>> for r2d2::Error {
    fn into(self) -> Answer<T, E> {
        Answer::unexpected(format!("Database error: {}", self.to_string()))
    }
}

pub async fn card_set_preview(
    body: web::Json<SetPreviewBody>,
    poll: web::Data<PgPool>,
    path: web::Path<CardPath>,
) -> Result<Answer<Example, SetPreviewError>, AWError> {
    let conn = poll.get().expect("failed to connect");

    use diesel::*;
    use schema::cards::dsl::*;

    let target = cards.filter(id.eq(path.card_id as i32));

    let query = diesel::update(target).set(preview_url.eq(Some(body.screenshot.to_string())));

    match query.execute(&conn) {
        Err(_) => Ok(Answer::fail(SetPreviewError::DatabaseFailure)),
        Ok(_) => Ok(Answer::ok(Example { id: path.card_id })),
    }
}

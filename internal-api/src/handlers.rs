use crate::answer::Answer;
use actix_web::{web, Error, HttpRequest};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use serde::{Serialize, Deserialize};

use howtocards_db::schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPreviewBody {
    card_id: u32,
    preview_relative_url: String,
}

pub async fn card_set_preview(
    body: web::Json<SetPreviewBody>,
    poll: web::Data<PgPool>,
) -> Result<Answer<String>, Error> {
    Ok(Answer::new(String::from("Hmmmm")))
}

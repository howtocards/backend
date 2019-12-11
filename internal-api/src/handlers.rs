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

pub async fn card_set_preview(
    body: web::Json<SetPreviewBody>,
    _poll: web::Data<PgPool>,
    path: web::Path<CardPath>,
) -> Result<Answer<Example>, Error> {
    println!("body: {:#?}", body.0);
    println!("card_id: {:#?}", path.card_id);
    Ok(Answer::new(Example { id: path.card_id}))
}

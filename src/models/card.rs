use chrono::NaiveDateTime;

use models::User;
use schema::cards;

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Default, Debug)]
#[belongs_to(User, foreign_key = "author_id")]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    /// Count of users, that added card to its library
    pub useful_for: i64,
}

#[derive(Debug, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "cards"]
#[serde(rename_all = "camelCase")]
pub struct CardNew {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

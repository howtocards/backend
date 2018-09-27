//! Diesel models

use chrono::NaiveDateTime;

use schema::cards;
use schema::tokens;
use schema::users;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct UserNew {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize, Insertable, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(token)]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Debug)]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Card {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "cards"]
pub struct CardNew {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

//! Diesel models
#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel::prelude::*;

use schema::cards;
use schema::tokens;
use schema::useful_marks;
use schema::users;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct UserNew {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize, Insertable, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(token)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

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

#[derive(Debug, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Card)]
pub struct UsefulMark {
    pub user_id: i32,
    pub card_id: i32,
    pub created_at: NaiveDateTime,
}

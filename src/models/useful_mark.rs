use crate::models::{Card, User};
use crate::schema::useful_marks;
use chrono::NaiveDateTime;
use diesel::{Associations, Insertable, Queryable};

#[derive(Debug, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Card)]
pub struct UsefulMark {
    pub user_id: i32,
    pub card_id: i32,
    pub created_at: NaiveDateTime,
}

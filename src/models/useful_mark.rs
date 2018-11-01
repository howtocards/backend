use chrono::NaiveDateTime;
use models::{Card, User};
use schema::useful_marks;

#[derive(Debug, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Card)]
pub struct UsefulMark {
    pub user_id: i32,
    pub card_id: i32,
    pub created_at: NaiveDateTime,
}

use crate::models::{Card, User};
use crate::schema::useful_marks;
use crate::time;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;

#[derive(Debug, Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Card)]
pub struct UsefulMark {
    pub user_id: i32,
    pub card_id: i32,
    pub created_at: NaiveDateTime,
}

impl UsefulMark {
    pub fn new(card_id: i32, user_id: i32) -> Self {
        UsefulMark {
            card_id,
            user_id,
            created_at: time::now(),
        }
    }

    pub fn create(conn: &PgConnection, card_id: i32, user_id: i32) -> Option<Self> {
        diesel::insert_into(useful_marks::table)
            .values(&Self::new(card_id, user_id))
            .get_result(conn)
            .ok()
    }

    pub fn delete(conn: &PgConnection, card_id: i32, user_id: i32) -> Option<usize> {
        let target = useful_marks::table
            .filter(useful_marks::card_id.eq(card_id))
            .filter(useful_marks::user_id.eq(user_id));

        diesel::delete(target).execute(conn).ok()
    }

    pub fn count_for_card(conn: &PgConnection, card_id: i32) -> i64 {
        use diesel::dsl::count;

        useful_marks::table
            .filter(useful_marks::card_id.eq(card_id))
            .select(count(useful_marks::card_id))
            .first(conn)
            .unwrap_or(0)
    }
}

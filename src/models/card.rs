use chrono::NaiveDateTime;

use diesel::prelude::*;
use models::User;
use schema::cards;

#[derive(Debug, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "cards"]
#[serde(rename_all = "camelCase")]
pub struct CardNew {
    pub author_id: i32,
    pub title: String,
    pub content: String,
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

impl Card {
    pub fn find_by_id(conn: &PgConnection, card_id: i32) -> Option<Self> {
        use schema::cards::dsl::*;

        cards.find(card_id).get_result::<Self>(conn).ok()
    }

    pub fn get_useful_for_user(conn: &PgConnection, user_id: i32) -> Vec<Self> {
        use schema::cards;
        use schema::useful_marks;

        cards::table
            .inner_join(useful_marks::table)
            .filter(useful_marks::user_id.eq(user_id))
            .select(cards::all_columns)
            .load::<Card>(conn)
            .unwrap_or(Vec::new())
    }
}

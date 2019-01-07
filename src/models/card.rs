use chrono::NaiveDateTime;

use crate::models::User;
use crate::schema::cards;
use crate::time;
use diesel::dsl::sql;
use diesel::prelude::*;

#[derive(Debug, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "cards"]
#[serde(rename_all = "camelCase")]
pub struct CardNew {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Queryable, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardMeta {
    pub is_useful: bool,
    pub can_edit: bool,
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
    pub meta: CardMeta,
}

pub type AllColumns = (
    crate::schema::cards::id,
    crate::schema::cards::author_id,
    crate::schema::cards::title,
    crate::schema::cards::content,
    crate::schema::cards::created_at,
    crate::schema::cards::updated_at,
    crate::schema::cards::useful_for,
    (
        diesel::expression::SqlLiteral<diesel::sql_types::Bool>,
        diesel::expression::SqlLiteral<diesel::sql_types::Bool>,
    ),
);

pub fn select_card(requester_id: i32) -> AllColumns {
    use crate::schema::cards::dsl::*;

    (
        id,
        author_id,
        title,
        content,
        created_at,
        updated_at,
        useful_for,
        (
            // Card is useful if useful_marks more than one
            sql(format!(
                "CASE WHEN (select count(*) from useful_marks WHERE user_id={} AND card_id=cards.id)=1 THEN true ELSE false END AS is_useful",
                requester_id
            ).as_str()),

            // User can edit card if he is author
            sql(format!(
                "CASE WHEN author_id={} THEN true ELSE false END AS can_edit",
                requester_id
            )
            .as_str()),
        ),
    )
}

impl Card {
    pub fn find_by_id(conn: &PgConnection, card_id: i32, current_user_id: i32) -> Option<Self> {
        cards::table
            .find(card_id)
            .select(select_card(current_user_id))
            .get_result(conn)
            .ok()
    }

    pub fn get_latest_cards(conn: &PgConnection, requester_id: i32) -> Vec<Self> {
        cards::table
            .select(select_card(requester_id))
            .order(cards::created_at.desc())
            .get_results(conn)
            .unwrap_or_default()
    }

    pub fn get_useful_for_user(conn: &PgConnection, user_id: i32) -> Vec<Self> {
        use crate::schema::useful_marks;

        cards::table
            .inner_join(useful_marks::table)
            .order(useful_marks::created_at.desc())
            .filter(useful_marks::user_id.eq(user_id))
            .select(select_card(user_id))
            .load(conn)
            .unwrap_or_default()
    }

    pub fn find_all_by_author(conn: &PgConnection, author_id: i32) -> Vec<Self> {
        cards::table
            .order(cards::updated_at.desc())
            .filter(cards::author_id.eq(author_id))
            .select(select_card(author_id))
            .get_results(conn)
            .unwrap_or_default()
    }

    pub fn create(conn: &PgConnection, new_card: CardNew, creator_id: i32) -> Option<Self> {
        diesel::insert_into(cards::table)
            .values(&new_card)
            .returning(select_card(creator_id))
            .get_result(conn)
            .ok()
    }

    pub fn delete(conn: &PgConnection, card_id: i32, requester_id: i32) -> Option<Self> {
        let target = cards::table
            .filter(cards::id.eq(card_id))
            .filter(cards::author_id.eq(requester_id));

        diesel::delete(target)
            .returning(select_card(requester_id))
            .get_result(conn)
            .ok()
    }

    pub fn update(
        conn: &PgConnection,
        card_id: i32,
        requester_id: i32,
        title: String,
        content: String,
    ) -> Option<Card> {
        let target = cards::table.filter(cards::id.eq(card_id));

        diesel::update(target)
            .set((
                cards::updated_at.eq(Some(time::now())),
                cards::title.eq(title),
                cards::content.eq(content),
            ))
            .returning(select_card(requester_id))
            .get_result(conn)
            .ok()
    }

    pub fn update_useful_for(
        conn: &PgConnection,
        card_id: i32,
        useful_for: i64,
        requester_id: i32,
    ) -> Option<Self> {
        let target = cards::table.filter(cards::id.eq(card_id));

        diesel::update(target)
            .set((
                cards::updated_at.eq(Some(time::now())),
                cards::useful_for.eq(useful_for),
            ))
            .returning(select_card(requester_id))
            .get_result(conn)
            .ok()
    }
}

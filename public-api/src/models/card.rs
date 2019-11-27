use chrono::NaiveDateTime;

use crate::models::User;
use crate::slate::plain_serialize;
use crate::time;
use crate::views::CardMeta as CardMetaView;
use diesel::dsl::sql;
use diesel::prelude::*;
use howtocards_db::schema::cards;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardNew {
    pub author_id: i32,
    pub title: String,
    pub content: Value,
}

impl Into<CardNewForSearch> for CardNew {
    fn into(self) -> CardNewForSearch {
        let content_for_search = plain_serialize(&self.content);
        CardNewForSearch {
            author_id: self.author_id,
            title: self.title,
            content: self.content,
            content_for_search,
        }
    }
}

#[derive(Debug, Deserialize, Insertable, Associations)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "cards"]
#[serde(rename_all = "camelCase")]
pub struct CardNewForSearch {
    pub author_id: i32,
    pub title: String,
    pub content: Value,
    pub content_for_search: String,
}

#[derive(Serialize, Deserialize, Queryable, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
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
    pub content: Value,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    /// Count of users, that added card to its library
    pub useful_for: i64,
    pub permissions: Permissions,
    #[serde(skip)]
    pub content_for_search: String,
    pub tags: Vec<String>,
    pub preview_url: Option<String>,
}

pub type AllColumns = (
    howtocards_db::schema::cards::id,
    howtocards_db::schema::cards::author_id,
    howtocards_db::schema::cards::title,
    howtocards_db::schema::cards::content,
    howtocards_db::schema::cards::created_at,
    howtocards_db::schema::cards::updated_at,
    howtocards_db::schema::cards::useful_for,
    // permissions
    (
        diesel::expression::SqlLiteral<diesel::sql_types::Bool>,
        diesel::expression::SqlLiteral<diesel::sql_types::Bool>,
    ),
    howtocards_db::schema::cards::content_for_search,
    howtocards_db::schema::cards::tags,
    howtocards_db::schema::cards::preview_url,
);

impl Card {
    #[inline]
    pub fn all_columns(requester_id: i32) -> AllColumns {
        use howtocards_db::schema::cards::dsl::*;

        (
            id,
            author_id,
            title,
            content,
            created_at,
            updated_at,
            useful_for,
            // permissions
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
            content_for_search,
            tags,
            preview_url,
        )
    }

    pub fn select_for(requester_id: i32) -> diesel::dsl::Select<cards::table, AllColumns> {
        use howtocards_db::schema::cards::dsl::*;

        cards.select(Self::all_columns(requester_id))
    }

    pub fn find_by_id(conn: &PgConnection, card_id: i32, requester_id: i32) -> Option<Self> {
        Self::select_for(requester_id)
            .find(card_id)
            .get_result(conn)
            .ok()
    }

    pub fn get_latest_cards(conn: &PgConnection, requester_id: i32) -> Vec<Self> {
        Self::select_for(requester_id)
            .order(cards::created_at.desc())
            .get_results(conn)
            .unwrap_or_default()
    }

    pub fn get_useful_for_user(conn: &PgConnection, user_id: i32) -> Vec<Self> {
        use howtocards_db::schema::useful_marks;

        Self::select_for(user_id)
            .inner_join(useful_marks::table)
            .order(useful_marks::created_at.desc())
            .filter(useful_marks::user_id.eq(user_id))
            .load(conn)
            .unwrap_or_default()
    }

    pub fn find_all_by_author(conn: &PgConnection, author_id: i32) -> Vec<Self> {
        Self::select_for(author_id)
            .order(cards::updated_at.desc())
            .filter(cards::author_id.eq(author_id))
            .get_results(conn)
            .unwrap_or_default()
    }

    pub fn create(conn: &PgConnection, new_card: CardNew, creator_id: i32) -> Option<Self> {
        let card: CardNewForSearch = new_card.into();
        diesel::insert_into(cards::table)
            .values(&card)
            .returning(Self::all_columns(creator_id))
            .get_result(conn)
            .ok()
    }

    pub fn delete(conn: &PgConnection, card_id: i32, requester_id: i32) -> Option<Self> {
        let target = cards::table
            .filter(cards::id.eq(card_id))
            .filter(cards::author_id.eq(requester_id));

        diesel::delete(target)
            .returning(Self::all_columns(requester_id))
            .get_result(conn)
            .ok()
    }

    pub fn update(
        conn: &PgConnection,
        card_id: i32,
        requester_id: i32,
        title: String,
        content: Value,
    ) -> Option<Card> {
        let target = cards::table.filter(cards::id.eq(card_id));
        let content_for_search = plain_serialize(&content);

        diesel::update(target)
            .set((
                cards::updated_at.eq(Some(time::now())),
                cards::title.eq(title),
                cards::content.eq(content),
                cards::content_for_search.eq(content_for_search),
            ))
            .returning(Self::all_columns(requester_id))
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
            .returning(Self::all_columns(requester_id))
            .get_result(conn)
            .ok()
    }

    pub fn encodable_meta(self) -> CardMetaView {
        CardMetaView {
            id: self.id,
            title: self.title,
            description: self.content_for_search,
            created_at: self.created_at,
            updated_at: self.updated_at,
            preview: None,
        }
    }
}

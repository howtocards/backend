//! Search

use actix_base::prelude::*;
use diesel::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use howtocards_db::schema::cards;

#[allow(dead_code)]
pub enum Sort {
    RecentCreated,
    MostUseful,
}

pub struct SearchRequest {
    pub requester_id: Option<i32>,
    pub query: String,
    pub pagination: Pagination,
}

impl Message for SearchRequest {
    type Result = Option<Vec<Card>>;
}

impl Handler<SearchRequest> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: SearchRequest, _ctx: &mut Self::Context) -> Self::Result {
        let mut query = cards::table
            .select(Card::all_columns(msg.requester_id.unwrap_or(-1)))
            .into_boxed();

        let query_for_title = msg.query.replace(" ", "%");
        let query_string = format!("%{}%", query_for_title);

        query = query.filter(
            cards::content_for_search
                .ilike::<String>(query_string.clone())
                .or(cards::title.ilike::<String>(query_string))
                .or(cards::title.eq::<String>(query_for_title)),
        );

        let cards = query.load::<Card>(&self.conn);
        cards.ok()
    }
}

pub const DEFAULT_PAGINATION_COUNT: u32 = 10;
pub const MAXIMUM_PAGINATION_COUNT: u32 = 200;

pub struct Pagination {
    pub page: u32,
    pub count: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: 1,
            count: DEFAULT_PAGINATION_COUNT,
        }
    }
}

#[allow(dead_code)]
impl Pagination {
    fn offset(&self) -> u32 {
        self.page * self.limit() - self.limit()
    }

    fn limit(&self) -> u32 {
        if self.count < MAXIMUM_PAGINATION_COUNT {
            self.count
        } else {
            MAXIMUM_PAGINATION_COUNT
        }
    }
}

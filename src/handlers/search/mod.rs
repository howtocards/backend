//! Search

use actix_base::prelude::*;
use actix_web::*;

use crate::app_state::DbExecutor;
use crate::models::*;
use crate::prelude::*;

pub struct SearchRequest {
    pub requester_id: i32,
    pub query: String,
    pub pagination: Pagination,
}

impl Message for SearchRequest {
    type Result = Option<Vec<Card>>;
}

impl Handler<SearchRequest> for DbExecutor {
    type Result = Option<Vec<Card>>;

    fn handle(&mut self, msg: SearchRequest, _ctx: &mut Self::Context) -> Self::Result {
        None
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

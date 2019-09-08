//! /search

use crate::app_state::AppState;
use crate::auth::AuthOptional;
use crate::handlers::search::*;
use crate::models::*;
use crate::prelude::*;
use actix_web::{Query, State};

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
    page: Option<u32>,
    count: Option<u32>,
}

impl SearchQuery {
    fn to_pagination(&self) -> Pagination {
        let def: Pagination = Default::default();

        Pagination {
            page: self.page.unwrap_or(def.page),
            count: self.count.unwrap_or(def.count),
        }
    }
}

/// GET /search/?
pub fn search(auth: AuthOptional, state: State<AppState>, query: Query<SearchQuery>) -> FutRes {
    #[derive(Serialize)]
    struct R {
        cards: Vec<Card>,
    }

    state
        .pg
        .send(SearchRequest {
            requester_id: auth.user.map(|u| u.id),
            pagination: query.to_pagination(),
            query: query.q.clone(),
        })
        .from_err()
        .and_then(|cards| {
            Ok(answer_success!(
                Ok,
                R {
                    cards: cards.unwrap_or_default()
                }
            ))
        })
        .responder()
}

#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope.resource("/", |r| {
        r.get().with(self::search);
    })
}

use actix::prelude::*;
use actix_web::*;
use failure::*;
use futures::*;
use juniper::http::graphiql::graphiql_source;
use actix_web::Error as ActixError;

use app_state::{AppState, Req};
use auth::{Auth, AuthOptional};
use handlers::gql::GraphQLData;


fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, ActixError> {
    let html = graphiql_source("http://127.0.0.1:9000/api/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql(
    (st, data): (State<AppState>, Json<GraphQLData>),
) -> FutureResponse<HttpResponse> {
    st.gql
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}


#[inline]
pub fn scope(scope: Scope<AppState>) -> Scope<AppState> {
    scope
        .resource("/graphql", |r| {
            r.post().with(self::graphql)
        }).resource("/graphiql", |r| {
            r.get().h(self::graphiql)
        })
}



use serde_json;
use actix::prelude::*;
use actix_web::Error;
use juniper::http::GraphQLRequest;

use graphql::*;
use app_state::GraphQLExecutor;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
  type Result = Result<String, Error>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _ctx: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let text = serde_json::to_string(&res)?;
        Ok(text)
    }
}

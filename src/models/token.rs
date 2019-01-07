use crate::models::User;
use crate::schema::tokens;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize, Insertable, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(token)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

impl Token {
    pub fn generate() -> String {
        format!("{}-{}", Uuid::new_v4(), Uuid::new_v4())
    }

    pub fn new(user_id: i32) -> Self {
        Token {
            token: Self::generate(),
            user_id,
        }
    }

    pub fn create(conn: &PgConnection, user_id: i32) -> Option<Self> {
        let token = Self::new(user_id);

        diesel::insert_into(tokens::table)
            .values(&token)
            .get_result(conn)
            .ok()
    }
}

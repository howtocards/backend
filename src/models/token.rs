use models::User;
use schema::tokens;

#[derive(Debug, Queryable, Serialize, Insertable, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(token)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

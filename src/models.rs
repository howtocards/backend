use schema::users;
use schema::tokens;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(token)]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

use schema::users;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

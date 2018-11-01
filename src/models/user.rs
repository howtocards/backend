use schema::users;
use views::{EncodableUserPrivate, EncodableUserPublic};

#[derive(Queryable, Associations, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

impl User {
    /// Converts this User model into an public for serialization
    pub fn encodable_public(self) -> EncodableUserPublic {
        let User {
            id, display_name, ..
        } = self;
        EncodableUserPublic { id, display_name }
    }

    /// Converts this User model into an private for serialization
    pub fn encodable_private(self) -> EncodableUserPrivate {
        let User {
            display_name,
            email,
            id,
            ..
        } = self;
        EncodableUserPrivate {
            display_name,
            email,
            id,
        }
    }
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct UserNew {
    pub email: String,
    pub password: String,
}

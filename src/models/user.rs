use crate::schema::users;
use crate::views::{EncodableUserPrivate, EncodableUserPublic, UserSettings};
use diesel::prelude::*;

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

    pub fn encodable_settings(self) -> UserSettings {
        let User { display_name, .. } = self;

        UserSettings {
            display_name,
            gravatar_email: None,
        }
    }

    pub fn find_by_id(conn: &PgConnection, user_id: i32) -> Option<Self> {
        use crate::schema::users::dsl::*;

        users.find(user_id).get_result::<Self>(conn).ok()
    }

    pub fn create(conn: &PgConnection, new_user: UserNew) -> Option<Self> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .ok()
    }

    pub fn find_by_credentials(conn: &PgConnection, credentials: UserNew) -> Option<Self> {
        users::table
            .filter(users::email.eq(credentials.email))
            .filter(users::password.eq(credentials.password))
            .get_result(conn)
            .ok()
    }

    pub fn find_by_token(conn: &PgConnection, token: String) -> Option<Self> {
        use crate::schema::tokens;

        tokens::table
            .inner_join(users::table)
            .filter(tokens::token.eq(token))
            .select(users::all_columns)
            .get_result(conn)
            .ok()
    }

    pub fn update(conn: &PgConnection, user_id: i32, display_name: String) -> Option<User> {
        let target = users::table.filter(users::id.eq(user_id));

        diesel::update(target)
            .set(users::display_name.eq(Some(display_name)))
            .returning(users::all_columns)
            .get_result(conn)
            .ok()
    }
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct UserNew {
    pub email: String,
    pub password: String,
}

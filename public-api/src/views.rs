use chrono::NaiveDateTime;

/// Serialization for User model
/// Without password field
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodableUserPrivate {
    pub display_name: Option<String>,
    pub email: String,
    pub id: i32,
    pub avatar: String,
    pub username: String,
}

/// Serialization for User model
/// Same as Private except without email
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodableUserPublic {
    pub display_name: Option<String>,
    pub id: i32,
    pub avatar: String,
    pub username: String,
}

/// User settings to communicate with frontend
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub display_name: Option<String>,
    pub gravatar_email: Option<String>,
    pub current_email: Option<String>,
    pub username: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMeta {
    pub title: String,
    pub description: String,
    pub id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub preview: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    pub is_useful: bool,
    pub can_edit: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: CardContent,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    /// Count of users, that added card to its library
    pub useful_for: i64,
    pub permissions: Permissions,
    pub tags: Vec<String>,
    pub preview_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CardContent {
    /// Source returned only if card is not rendered yet
    Source(serde_json::Value),
    Rendered(String),
}

use chrono::NaiveDateTime;

/// Serialization for User model
/// Without password field
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodableUserPrivate {
    pub display_name: Option<String>,
    pub email: String,
    pub id: i32,
}

/// Serialization for User model
/// Same as Private except without email
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodableUserPublic {
    pub display_name: Option<String>,
    pub id: i32,
}

/// User settings to communicate with frontend
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub display_name: Option<String>,
    pub gravatar_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardMeta {
    pub title: String,
    pub description: String,
    pub id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub preview: Option<String>,
}

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

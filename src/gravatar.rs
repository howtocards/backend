
pub fn create_avatar_url(email: String) -> String {
    format!("https://www.gravatar.com/avatar/{:x}?rating=g&d=retro", md5::compute(email))
}

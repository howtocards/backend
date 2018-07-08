use super::Tokens;
use super::Users;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tables {
    pub users: Users,
    pub tokens: Tokens,
}

impl Default for Tables {
    fn default() -> Tables {
        Tables {
            users: Default::default(),
            tokens: Default::default(),
        }
    }
}

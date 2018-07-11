use super::{Tokens, Users};
use db::Indexable;
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

impl Indexable for Tables {
    fn reindex(&mut self) {
        self.users.reindex();
        self.tokens.reindex();
    }
}

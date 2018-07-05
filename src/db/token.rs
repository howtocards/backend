use db::indexable::Indexable;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    /// all tokens
    /// (token, user_id)
    tokens: BTreeMap<String, u32>,

    /// index save token by user id
    /// (user_id, Set{token})
    #[serde(skip_serializing)]
    by_user_id: BTreeMap<u32, BTreeSet<String>>,
}

impl Default for Tokens {
    fn default() -> Tokens {
        Tokens {
            tokens: BTreeMap::new(),
            by_user_id: BTreeMap::new(),
        }
    }
}

impl Indexable for Tokens {
    fn reindex(&mut self) {
        self.by_user_id = BTreeMap::new();

        for (token, user_id) in &self.tokens {
            if !self.by_user_id.contains_key(user_id) {
                let mut tokens = BTreeSet::<String>::new();

                tokens.insert(token.clone());
                self.by_user_id.insert(*user_id, tokens);
            } else {
                let mut tokens = self.by_user_id.get_mut(user_id).unwrap();

                tokens.insert(token.clone());
            }
        }
    }
}

impl Tokens {
    fn new() -> Tokens {
        Tokens { ..Default::default() }
    }

    pub fn insert(&mut self, user_id: u32, token: String) {
        self.tokens.insert(token, user_id);
    }
}

use db::Indexable;
use ron;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct Card {
    pub id: u32,
    pub title: String,
    pub content: String,
    /// email
    pub author_id: u32,
    pub created_at: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cards {
    /// card.id -> card
    cards: HashMap<u32, Card>,
    last_id: u32,

    /// user.id -> card.id[]
    #[serde(skip)]
    cards_by_user_id: HashMap<u32, Vec<u32>>,
}

impl Indexable for Cards {}

impl Cards {
    fn next_seq_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn get(&self, id: u32) -> Option<&Card> {
        self.cards.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Card> {
        self.cards.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Card> {
        let result = self.cards.remove(&id);
        self.reindex();
        result
    }

    pub fn update(&mut self, id: u32, card: Card) -> Option<Card> {
        let result = self.cards.insert(id, card);
        self.reindex();
        result
    }

    pub fn create(&mut self, user: Card) -> Option<Card> {
        let mut clone = user.clone();

        clone.id = self.next_seq_id();

        let result = self.cards.insert(clone.id, clone);
        self.reindex();
        result
    }
}

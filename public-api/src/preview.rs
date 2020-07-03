use failure::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct PreviewCard {
    card: String,
    callback: String,
    extra: HashMap<String, String>,
}

pub fn create_for_card(card_id: u32, preview_queue_url: String) -> Result<(), Error> {
    reqwest::Client::new()
        .post(&format!("{}/render/card", preview_queue_url))
        .json(&PreviewCard {
            card: card_id.to_string(),
            callback: format!("/preview/card/{}", card_id),
            extra: Default::default(),
        })
        .send()
        .map(|_| {})
        .map_err(|e| e.into())
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub description_zh: String,
    pub starting_artifacts: Vec<String>,
}

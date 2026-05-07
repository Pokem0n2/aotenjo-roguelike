use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternDef {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub name_ja: String,
    pub rarity: super::artifact::Rarity,
    pub base_fan: f64,
    pub inheritance: Vec<String>,
    pub validate_fn: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub name_zh: String,
    pub base_fan: f64,
    pub rarity: super::artifact::Rarity,
}

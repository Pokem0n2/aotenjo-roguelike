use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub description_zh: String,
    pub rarity: Rarity,
    pub cost: u32,
    pub sell_value: u32,
    pub effect: ArtifactEffect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArtifactEffect {
    AddFu(u64),
    AddFan(f64),
    MultiplyMult(f64),
    ConditionalAddFan { condition: String, value: f64 },
    ScalingAddFu { per_round: u64, current: u64 },
    ScalingAddFan { per_round: f64, current: f64 },
    PatternBonus { pattern_id: String, bonus: f64 },
}

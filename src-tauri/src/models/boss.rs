use serde::{Deserialize, Serialize};

use super::tile::TileSuit;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Boss {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub gimmick: BossGimmick,
    pub score_multiplier: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BossGimmick {
    BlindSuit(TileSuit),
    ReducedPlays(u8),
    NoDiscard,
    TaxPerPlay(u32),
    ScoreThresholdScale(f64),
    ForcedPattern(String),
}

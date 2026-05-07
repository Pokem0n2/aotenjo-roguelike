use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreComponents {
    pub base_fu: u64,
    pub tile_fu: u64,
    pub artifact_fu: u64,
    pub base_fan: f64,
    pub artifact_fan: f64,
    pub cumulative_mult: f64,
}

impl ScoreComponents {
    pub fn final_score(&self) -> u64 {
        let total_fu = self.base_fu + self.tile_fu + self.artifact_fu;
        let total_fan = self.base_fan + self.artifact_fan;
        (total_fu as f64 * total_fan * self.cumulative_mult) as u64
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoreResult {
    pub fu: u64,
    pub fan: f64,
    pub mult: f64,
    pub final_score: u64,
    pub breakdown: Vec<String>,
}

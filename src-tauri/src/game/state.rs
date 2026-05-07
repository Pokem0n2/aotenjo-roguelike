use serde::{Deserialize, Serialize};

use crate::models::tile::Tile;
use crate::models::wall::Wall;
use crate::models::hand::Play;
use crate::models::artifact::Artifact;
use crate::models::gadget::Gadget;
use crate::models::boss::Boss;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GamePhase {
    MainMenu,
    DeckSelect,
    RoundIntro,
    Playing,
    PlayScoring,
    Shop,
    RoundResult,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub phase: GamePhase,
    pub run_seed: u64,
    pub current_wind: Wind,
    pub current_round: u8,
    pub current_play: u8,
    pub wall: Wall,
    pub hand_tiles: Vec<Tile>,
    pub selected_tile_ids: Vec<u32>,
    pub plays_made: Vec<PlayResult>,
    pub round_score: u64,
    pub round_target: u64,
    pub artifacts: Vec<Artifact>,
    pub gadgets: Vec<Gadget>,
    pub currency: u32,
    pub boss: Option<Boss>,
    pub max_plays: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayResult {
    pub play: Play,
    pub score: u64,
    pub patterns_matched: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::MainMenu,
            run_seed: 0,
            current_wind: Wind::East,
            current_round: 1,
            current_play: 0,
            wall: Wall::new(0),
            hand_tiles: Vec::new(),
            selected_tile_ids: Vec::new(),
            plays_made: Vec::new(),
            round_score: 0,
            round_target: 0,
            artifacts: Vec::new(),
            gadgets: Vec::new(),
            currency: 0,
            boss: None,
            max_plays: 4,
        }
    }

    pub fn start_run(&mut self, seed: u64) {
        self.run_seed = seed;
        self.phase = GamePhase::RoundIntro;
        self.current_wind = Wind::East;
        self.current_round = 1;
        self.current_play = 0;
        self.currency = 0;
        self.artifacts.clear();
        self.gadgets.clear();
        self.start_round(seed);
    }

    pub fn start_round(&mut self, seed: u64) {
        self.wall = Wall::new(seed + (self.current_round as u64 * 1000) + (self.current_wind as u64 * 10000));
        self.hand_tiles = self.wall.draw(14);
        self.selected_tile_ids.clear();
        self.plays_made.clear();
        self.round_score = 0;
        self.current_play = 0;
        self.round_target = self.get_target_score();
        self.phase = GamePhase::Playing;
    }

    fn get_target_score(&self) -> u64 {
        let wind_base = match self.current_wind {
            Wind::East => 0,
            Wind::South => 4,
            Wind::West => 8,
            Wind::North => 12,
        };
        let round_index = wind_base + (self.current_round - 1) as u32;

        let targets = [
            300, 800, 2000, 5000,       // East
            12000, 30000, 80000, 200000, // South
            500000, 1500000, 5000000, 15000000, // West
            50000000, 200000000, 800000000, 3000000000, // North
        ];

        targets.get(round_index as usize).copied().unwrap_or(5000000000)
    }

    pub fn draw_more_tiles(&mut self, count: usize) -> Vec<Tile> {
        let tiles = self.wall.draw(count);
        self.hand_tiles.extend(tiles.iter().cloned());
        tiles
    }

    pub fn select_tile(&mut self, tile_id: u32) {
        if let Some(pos) = self.selected_tile_ids.iter().position(|&id| id == tile_id) {
            self.selected_tile_ids.remove(pos);
        } else {
            self.selected_tile_ids.push(tile_id);
        }
    }

    pub fn get_selected_tiles(&self) -> Vec<Tile> {
        self.hand_tiles
            .iter()
            .filter(|t| self.selected_tile_ids.contains(&t.id))
            .copied()
            .collect()
    }

    pub fn submit_play(&mut self) -> Result<PlayResult, String> {
        let selected = self.get_selected_tiles();

        // Validate play structure
        let play = crate::models::hand::validate_play_structure(&selected)?;

        // Calculate score
        let score = self.calculate_play_score(&play);

        // Remove played tiles from hand (except pair)
        let played_ids: Vec<u32> = play.all_tiles().iter().map(|t| t.id).collect();
        self.hand_tiles.retain(|t| !played_ids.contains(&t.id));

        let result = PlayResult {
            play,
            score,
            patterns_matched: Vec::new(),
        };

        self.round_score += result.score;
        self.plays_made.push(result.clone());
        self.selected_tile_ids.clear();
        self.current_play += 1;

        // Draw replacement tiles
        let tiles_to_draw = std::cmp::min(3, self.wall.remaining());
        if tiles_to_draw > 0 {
            self.draw_more_tiles(tiles_to_draw);
        }

        Ok(result)
    }

    fn calculate_play_score(&self, play: &Play) -> u64 {
        let tiles = play.all_tiles();
        let mut total_fu: u64 = 0;
        for tile in &tiles {
            total_fu += tile.base_fu() + (tile.buff_fu.max(0) as u64);
        }

        // Base fan from hand structure
        let base_fan: f64 = match play.melds.len() {
            0 => 1.0, // Special hand
            _ => {
                let mut fan = 1.0;
                for meld in &play.melds {
                    match meld.kind {
                        crate::models::hand::MeldKind::Pon => fan += 1.0,
                        crate::models::hand::MeldKind::Kan => fan += 2.0,
                        crate::models::hand::MeldKind::Chi => fan += 0.5,
                        crate::models::hand::MeldKind::Pair => {}
                    }
                }
                fan
            }
        };

        // Apply artifact effects
        let mut artifact_fan = 0.0f64;
        let mut artifact_fu = 0u64;
        let mut cumulative_mult = 1.0f64;

        for artifact in &self.artifacts {
            match &artifact.effect {
                crate::models::artifact::ArtifactEffect::AddFu(v) => artifact_fu += v,
                crate::models::artifact::ArtifactEffect::AddFan(v) => artifact_fan += v,
                crate::models::artifact::ArtifactEffect::MultiplyMult(v) => cumulative_mult *= v,
                _ => {}
            }
        }

        let total_fu_final = total_fu + artifact_fu;
        let total_fan_final = base_fan + artifact_fan;

        (total_fu_final as f64 * total_fan_final * cumulative_mult) as u64
    }

    pub fn is_round_over(&self) -> bool {
        self.current_play >= self.max_plays || self.round_score >= self.round_target
    }

    pub fn end_round(&mut self) -> RoundOutcome {
        if self.round_score >= self.round_target {
            let overkill_ratio = self.round_score as f64 / self.round_target as f64;
            let bonus = self.calculate_overkill_bonus(overkill_ratio);
            self.currency += bonus;

            self.current_round += 1;
            if self.current_round > 4 {
                self.current_round = 1;
                self.current_wind = match self.current_wind {
                    Wind::East => Wind::South,
                    Wind::South => Wind::West,
                    Wind::West => Wind::North,
                    Wind::North => {
                        self.phase = GamePhase::Victory;
                        return RoundOutcome::Victory;
                    }
                };
            }

            RoundOutcome::Pass {
                score: self.round_score,
                target: self.round_target,
                bonus,
            }
        } else {
            self.phase = GamePhase::GameOver;
            RoundOutcome::Fail {
                score: self.round_score,
                target: self.round_target,
            }
        }
    }

    fn calculate_overkill_bonus(&self, ratio: f64) -> u32 {
        if ratio >= 5.0 {
            500
        } else if ratio >= 4.0 {
            300
        } else if ratio >= 3.0 {
            200
        } else if ratio >= 2.0 {
            100
        } else {
            50
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoundOutcome {
    Pass { score: u64, target: u64, bonus: u32 },
    Fail { score: u64, target: u64 },
    Victory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_new() {
        let state = GameState::new();
        assert_eq!(state.phase, GamePhase::MainMenu);
    }

    #[test]
    fn test_start_run() {
        let mut state = GameState::new();
        state.start_run(42);
        assert_eq!(state.phase, GamePhase::Playing);
        assert_eq!(state.hand_tiles.len(), 14);
        assert_eq!(state.wall.remaining(), 122);
    }

    #[test]
    fn test_target_score_progression() {
        let mut state = GameState::new();
        state.start_run(42);
        state.current_wind = Wind::East;
        state.current_round = 1;
        assert_eq!(state.get_target_score(), 300);

        state.current_round = 2;
        assert_eq!(state.get_target_score(), 800);

        state.current_wind = Wind::South;
        state.current_round = 1;
        assert_eq!(state.get_target_score(), 12000);
    }
}

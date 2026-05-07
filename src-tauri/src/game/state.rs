use serde::{Deserialize, Serialize};

use crate::models::tile::Tile;
use crate::models::wall::Wall;
use crate::models::hand::Play;
use crate::models::artifact::Artifact;
use crate::models::gadget::Gadget;
use crate::models::boss::Boss;
use crate::models::scoring::ScoreResult;

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
    pub score_detail: ScoreResult,
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
            300, 800, 2000, 5000,
            12000, 30000, 80000, 200000,
            500000, 1500000, 5000000, 15000000,
            50000000, 200000000, 800000000, 3000000000,
        ];

        targets.get(round_index as usize).copied().unwrap_or(5000000000)
    }

    pub fn draw_more_tiles(&mut self, count: usize) -> Vec<Tile> {
        let tiles = self.wall.draw(count);
        self.hand_tiles.extend(tiles.iter().cloned());
        tiles
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

        let play = crate::models::hand::validate_play_structure(&selected)?;

        // Gather all previously played tiles for context
        let all_played: Vec<Tile> = self.plays_made.iter()
            .flat_map(|pr| pr.play.all_tiles().into_iter().copied())
            .collect();

        let score_detail = crate::game::scoring_engine::calculate_score(
            &play,
            &self.artifacts,
            &all_played,
            &self.hand_tiles,
        );

        let patterns_matched: Vec<String> = {
            let play_tiles: Vec<Tile> = play.all_tiles().into_iter().copied().collect();
            crate::game::pattern_checker::check_patterns(&play_tiles, &all_played, &self.hand_tiles)
                .into_iter()
                .map(|pm| pm.name_zh.clone())
                .collect()
        };

        let played_ids: Vec<u32> = play.all_tiles().iter().map(|t| t.id).collect();
        self.hand_tiles.retain(|t| !played_ids.contains(&t.id));

        let score = score_detail.final_score;
        let result = PlayResult {
            play,
            score,
            patterns_matched,
            score_detail,
        };

        self.round_score += result.score;
        self.plays_made.push(result.clone());
        self.selected_tile_ids.clear();
        self.current_play += 1;

        // Update scaling artifacts after each play
        for artifact in &mut self.artifacts {
            if let crate::models::artifact::ArtifactEffect::ScalingAddFu { per_round, ref mut current } = artifact.effect {
                *current += per_round;
            }
            if let crate::models::artifact::ArtifactEffect::ScalingAddFan { per_round, ref mut current } = artifact.effect {
                *current += per_round;
            }
        }

        let tiles_to_draw = std::cmp::min(3, self.wall.remaining());
        if tiles_to_draw > 0 {
            self.draw_more_tiles(tiles_to_draw);
        }

        Ok(result)
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
        let base = match self.current_wind {
            Wind::East => 50,
            Wind::South => 100,
            Wind::West => 200,
            Wind::North => 500,
        };
        let multiplier = if ratio >= 5.0 { 10 } else if ratio >= 4.0 { 6 } else if ratio >= 3.0 { 4 } else if ratio >= 2.0 { 2 } else { 1 };
        base * multiplier
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

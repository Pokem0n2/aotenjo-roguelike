use serde::{Deserialize, Serialize};

use crate::models::tile::Tile;
use crate::models::wall::Wall;
use crate::models::hand::Play;
use crate::models::artifact::Artifact;
use crate::models::gadget::Gadget;
use crate::models::boss::Boss;
use crate::models::scoring::ScoreResult;
use crate::game::shop::{ShopItem, generate_shop};
use crate::game::boss_effects::{get_boss_for_round, apply_boss_effects, boss_pattern_bonus};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

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
    pub tax_per_play: u32,
    pub shop_items: Vec<ShopItem>,
    pub shop_rerolls: u8,
    pub boss_disabled: bool,
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
            max_plays: 20,
            tax_per_play: 0,
            shop_items: Vec::new(),
            shop_rerolls: 2,
            boss_disabled: false,
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
        self.boss_disabled = false;
        self.start_round(seed);
    }

    pub fn start_round(&mut self, seed: u64) {
        let round_seed = seed
            + (self.current_round as u64 * 1000)
            + (self.current_wind as u64 * 10000);
        self.wall = Wall::new(round_seed);
        self.hand_tiles = self.wall.draw(14);
        self.selected_tile_ids.clear();
        self.plays_made.clear();
        self.round_score = 0;
        self.current_play = 0;
        self.boss_disabled = false;

        // Assign boss
        let wind_index = match self.current_wind {
            Wind::East => 0,
            Wind::South => 1,
            Wind::West => 2,
            Wind::North => 3,
        };
        self.boss = Some(get_boss_for_round(wind_index, self.current_round));

        // Apply boss effects to target and max plays
        let base_target = self.get_target_score();
        let (max_plays, target, tax) = apply_boss_effects(
            self.boss.as_ref().unwrap(),
            20,
            base_target,
        );
        self.round_target = target;
        self.max_plays = max_plays;
        self.tax_per_play = tax;

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

    /// Get round progress (0-15) for shop quality scaling
    pub fn round_progress(&self) -> u32 {
        let wind_base = match self.current_wind {
            Wind::East => 0,
            Wind::South => 4,
            Wind::West => 8,
            Wind::North => 12,
        };
        wind_base + (self.current_round - 1) as u32
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

        let mut score_detail = crate::game::scoring_engine::calculate_score(
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

        // Boss pattern bonus
        if !self.boss_disabled {
            if let Some(ref boss) = self.boss {
                let bonus = boss_pattern_bonus(boss, &patterns_matched);
                if bonus > 0.0 {
                    score_detail.fan += bonus;
                    score_detail.final_score = (score_detail.fu as f64 * score_detail.fan * score_detail.mult) as u64;
                    score_detail.breakdown.push(format!("Boss奖励: +{:.1}番", bonus));
                }
            }
        }

        // Boss tax
        if !self.boss_disabled && self.tax_per_play > 0 {
            let tax = std::cmp::min(self.tax_per_play, self.currency);
            self.currency -= tax;
            score_detail.breakdown.push(format!("Boss税收: -{}金币", tax));
        }

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
            if let crate::models::artifact::ArtifactEffect::ScalingAddFu { per_round: _, ref mut current } = artifact.effect {
                *current += 3;
            }
            if let crate::models::artifact::ArtifactEffect::ScalingAddFan { per_round: _, ref mut current } = artifact.effect {
                *current += 0.5;
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

    pub fn skip_play(&mut self) {
        // Check NoDiscard boss gimmick
        if !self.boss_disabled {
            if let Some(ref boss) = self.boss {
                if matches!(boss.gimmick, crate::models::boss::BossGimmick::NoDiscard) {
                    // Cannot skip under NoDiscard boss — just increment play
                    self.current_play += 1;
                    return;
                }
            }
        }

        self.current_play += 1;

        let draw = std::cmp::min(8, self.wall.remaining());
        if draw > 0 {
            self.draw_more_tiles(draw);
        }

        for artifact in &mut self.artifacts {
            if let crate::models::artifact::ArtifactEffect::ScalingAddFu { per_round: _, ref mut current } = artifact.effect {
                *current += 3;
            }
            if let crate::models::artifact::ArtifactEffect::ScalingAddFan { per_round: _, ref mut current } = artifact.effect {
                *current += 0.5;
            }
        }
    }

    pub fn start_next_round(&mut self) {
        self.start_round(self.run_seed);
    }

    pub fn end_round(&mut self) -> RoundOutcome {
        if self.round_score >= self.round_target {
            let overkill_ratio = self.round_score as f64 / self.round_target as f64;
            let bonus = self.calculate_overkill_bonus(overkill_ratio);
            self.currency += bonus;

            // Generate shop for passing the round
            let progress = self.round_progress();
            let mut rng = ChaCha8Rng::seed_from_u64(self.run_seed + progress as u64 * 99999);
            self.shop_items = generate_shop(progress, &mut rng);
            self.shop_rerolls = 2;

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

            self.phase = GamePhase::Shop;
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

    // ── Shop operations ──

    pub fn shop_buy(&mut self, index: usize) -> Result<String, String> {
        let item = self.shop_items.get_mut(index)
            .ok_or_else(|| "无效的商品位置".to_string())?;
        if item.sold {
            return Err("该商品已售出".to_string());
        }
        if self.currency < item.cost {
            return Err(format!("金币不足 (需要{}，当前{})", item.cost, self.currency));
        }

        self.currency -= item.cost;
        item.sold = true;

        match &item.item_type {
            ShopItemType::Artifact(artifact) => {
                let name = artifact.name_zh.clone();
                self.artifacts.push(artifact.clone());
                Ok(format!("购买了遗物: {}", name))
            }
            ShopItemType::Gadget(gadget) => {
                let name = gadget.name_zh.clone();
                self.gadgets.push(gadget.clone());
                Ok(format!("购买了道具: {}", name))
            }
        }
    }

    pub fn shop_sell_artifact(&mut self, artifact_id: &str) -> Result<String, String> {
        let idx = self.artifacts.iter().position(|a| a.id == artifact_id)
            .ok_or_else(|| "未找到该遗物".to_string())?;
        let artifact = self.artifacts.remove(idx);
        self.currency += artifact.sell_value;
        Ok(format!("出售了遗物: {} (+{}金币)", artifact.name_zh, artifact.sell_value))
    }

    pub fn shop_sell_gadget(&mut self, gadget_id: &str) -> Result<String, String> {
        let idx = self.gadgets.iter().position(|g| g.id == gadget_id)
            .ok_or_else(|| "未找到该道具".to_string())?;
        let gadget = self.gadgets.remove(idx);
        let sell_value = crate::game::gadget_effects::gadget_cost(&gadget) / 2;
        self.currency += sell_value;
        Ok(format!("出售了道具: {} (+{}金币)", gadget.name_zh, sell_value))
    }

    pub fn shop_reroll(&mut self) -> Result<(), String> {
        if self.shop_rerolls == 0 {
            return Err("没有剩余的重抽次数".to_string());
        }
        let cost = 20;
        if self.currency < cost {
            return Err(format!("金币不足 (需要{}重抽)", cost));
        }
        self.currency -= cost;
        self.shop_rerolls -= 1;

        let progress = self.round_progress();
        let mut rng = ChaCha8Rng::seed_from_u64(
            self.run_seed + progress as u64 * 77777 + (3 - self.shop_rerolls as u64) * 333
        );
        self.shop_items = generate_shop(progress, &mut rng);
        Ok(())
    }

    pub fn shop_leave(&mut self) {
        self.phase = GamePhase::RoundIntro;
        self.start_round(self.run_seed);
    }

    // ── Gadget usage ──

    pub fn use_gadget(&mut self, gadget_id: &str, tile_ids: Vec<u32>) -> Result<String, String> {
        let idx = self.gadgets.iter().position(|g| g.id == gadget_id)
            .ok_or_else(|| "未找到该道具".to_string())?;
        let gadget = self.gadgets.remove(idx);

        match gadget.effect {
            crate::models::gadget::GadgetEffect::DestroyTiles { count } => {
                let to_remove: Vec<u32> = tile_ids.into_iter().take(count).collect();
                let removed = to_remove.len();
                self.hand_tiles.retain(|t| !to_remove.contains(&t.id));
                Ok(format!("销毁了{}张牌", removed))
            }
            crate::models::gadget::GadgetEffect::SwapTiles => {
                if tile_ids.len() < 2 {
                    return Err("请选择至少2张牌进行交换".to_string());
                }
                let swap_ids: Vec<u32> = tile_ids.into_iter().take(2).collect();
                self.hand_tiles.retain(|t| !swap_ids.contains(&t.id));
                let new_tiles = self.wall.draw(2);
                self.hand_tiles.extend(new_tiles.iter().cloned());
                Ok("交换了2张牌".to_string())
            }
            crate::models::gadget::GadgetEffect::PeekWall { count } => {
                let peek: Vec<Tile> = self.wall.peek_remaining().iter().take(count).cloned().collect();
                let names: Vec<String> = peek.iter().map(|t| t.display_zh()).collect();
                Ok(format!("牌墙顶部: {}", names.join(", ")))
            }
            crate::models::gadget::GadgetEffect::TransformTile => {
                if tile_ids.is_empty() {
                    return Err("请选择1张牌进行变形".to_string());
                }
                let target_id = tile_ids[0];
                if let Some(tile) = self.hand_tiles.iter_mut().find(|t| t.id == target_id) {
                    let new_rank = if tile.rank <= 7 { tile.rank + 1 } else { 1 };
                    tile.rank = new_rank;
                    tile.id = tile.id.wrapping_add(1000);
                    Ok(format!("变形为: {}", tile.display_zh()))
                } else {
                    Err("未找到选中的牌".to_string())
                }
            }
            crate::models::gadget::GadgetEffect::BuffTile { fu_bonus, mult_bonus } => {
                for id in &tile_ids {
                    if let Some(tile) = self.hand_tiles.iter_mut().find(|t| t.id == *id) {
                        tile.buff_fu += fu_bonus;
                    }
                }
                let mut msg = Vec::new();
                if fu_bonus > 0 { msg.push(format!("+{}符", fu_bonus)); }
                if mult_bonus > 0 { msg.push(format!("+{}番", mult_bonus)); }
                Ok(format!("强化了{}张牌 ({})", tile_ids.len(), msg.join(", ")))
            }
            crate::models::gadget::GadgetEffect::DrawExtra { count } => {
                let drawn = std::cmp::min(count, self.wall.remaining());
                self.draw_more_tiles(drawn);
                Ok(format!("摸了{}张牌", drawn))
            }
            crate::models::gadget::GadgetEffect::DisableBoss => {
                self.boss_disabled = true;
                // Restore default max_plays and remove tax
                self.max_plays = 20;
                self.tax_per_play = 0;
                Ok("已移除Boss效果!".to_string())
            }
            crate::models::gadget::GadgetEffect::GainCurrency { amount } => {
                self.currency += amount;
                Ok(format!("获得{}金币!", amount))
            }
        }
    }
}

use crate::game::shop::ShopItemType;

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
        assert!(state.boss.is_some());
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

    #[test]
    fn test_shop_buy_sell() {
        let mut state = GameState::new();
        state.start_run(42);
        state.currency = 500;

        // Force a shop state
        let progress = state.round_progress();
        let mut rng = ChaCha8Rng::seed_from_u64(42 + progress as u64 * 99999);
        state.shop_items = generate_shop(progress, &mut rng);
        state.shop_rerolls = 2;

        let item_count = state.shop_items.len();
        assert!(item_count > 0);

        // Buy first affordable item
        for i in 0..item_count {
            if state.shop_items[i].cost <= state.currency {
                let result = state.shop_buy(i);
                assert!(result.is_ok());
                assert!(state.shop_items[i].sold);
                break;
            }
        }
    }

    #[test]
    fn test_shop_reroll() {
        let mut state = GameState::new();
        state.start_run(42);
        state.currency = 100;

        let progress = state.round_progress();
        let mut rng = ChaCha8Rng::seed_from_u64(42 + progress as u64 * 99999);
        state.shop_items = generate_shop(progress, &mut rng);
        state.shop_rerolls = 2;

        let old_ids: Vec<String> = state.shop_items.iter()
            .map(|i| match &i.item_type {
                ShopItemType::Artifact(a) => a.id.clone(),
                ShopItemType::Gadget(g) => g.id.clone(),
            })
            .collect();

        let result = state.shop_reroll();
        assert!(result.is_ok());

        let new_ids: Vec<String> = state.shop_items.iter()
            .map(|i| match &i.item_type {
                ShopItemType::Artifact(a) => a.id.clone(),
                ShopItemType::Gadget(g) => g.id.clone(),
            })
            .collect();

        // Rerolled items should differ (extremely likely with 20+ artifacts/gadgets)
        assert_ne!(old_ids, new_ids);
        assert_eq!(state.shop_rerolls, 1);
    }

    #[test]
    fn test_use_gadget_destroy() {
        let mut state = GameState::new();
        state.start_run(42);
        let tile_ids: Vec<u32> = state.hand_tiles.iter().take(3).map(|t| t.id).collect();
        let hand_before = state.hand_tiles.len();

        state.gadgets.push(crate::models::gadget::Gadget {
            id: "hammer".into(),
            name_zh: "锤子".into(),
            name_en: "Hammer".into(),
            description_zh: "销毁牌".into(),
            effect: crate::models::gadget::GadgetEffect::DestroyTiles { count: 3 },
        });

        let result = state.use_gadget("hammer", tile_ids);
        assert!(result.is_ok());
        assert!(state.hand_tiles.len() <= hand_before);
        assert!(state.gadgets.is_empty());
    }
}

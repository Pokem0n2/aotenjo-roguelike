use std::sync::Mutex;
use tauri::State;

use crate::game::state::GameState;
use crate::models::tile::Tile;
use crate::models::artifact::{Artifact, Rarity};

#[derive(serde::Serialize)]
pub struct GameStateView {
    pub phase: String,
    pub current_wind: String,
    pub current_round: u8,
    pub current_play: u8,
    pub max_plays: u8,
    pub hand_tiles: Vec<TileView>,
    pub wall_remaining: usize,
    pub round_score: u64,
    pub round_target: u64,
    pub currency: u32,
    pub artifacts: Vec<ArtifactView>,
}

#[derive(serde::Serialize)]
pub struct TileView {
    pub suit: String,
    pub rank: u8,
    pub id: u32,
    pub is_red: bool,
    pub display_zh: String,
    pub base_fu: u64,
}

impl From<&Tile> for TileView {
    fn from(tile: &Tile) -> Self {
        Self {
            suit: format!("{:?}", tile.suit),
            rank: tile.rank,
            id: tile.id,
            is_red: tile.is_red,
            display_zh: tile.display_zh(),
            base_fu: tile.base_fu(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ArtifactView {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub description_zh: String,
    pub rarity: String,
    pub sell_value: u32,
}

impl From<&Artifact> for ArtifactView {
    fn from(artifact: &Artifact) -> Self {
        Self {
            id: artifact.id.clone(),
            name_zh: artifact.name_zh.clone(),
            name_en: artifact.name_en.clone(),
            description_zh: artifact.description_zh.clone(),
            rarity: format!("{:?}", artifact.rarity),
            sell_value: artifact.sell_value,
        }
    }
}

#[tauri::command]
pub fn start_run(seed: Option<u64>, state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let seed = seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });
    state.start_run(seed);
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn draw_tiles(count: u32, state: State<'_, Mutex<GameState>>) -> Result<Vec<TileView>, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let tiles = state.draw_more_tiles(count as usize);
    Ok(tiles.iter().map(TileView::from).collect())
}

#[tauri::command]
pub fn get_game_state(state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn select_tiles_for_play(
    tile_ids: Vec<u32>,
    state: State<'_, Mutex<GameState>>,
) -> Result<Vec<TileView>, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.selected_tile_ids = tile_ids;
    let selected = state.get_selected_tiles();
    Ok(selected.iter().map(TileView::from).collect())
}

#[derive(serde::Serialize)]
pub struct PlayResultView {
    pub score: u64,
    pub round_score: u64,
    pub round_target: u64,
    pub round_over: bool,
    pub fu: u64,
    pub fan: f64,
    pub mult: f64,
    pub patterns: Vec<String>,
    pub breakdown: Vec<String>,
}

#[tauri::command]
pub fn submit_play(state: State<'_, Mutex<GameState>>) -> Result<PlayResultView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let result = state.submit_play()?;
    let detail = &result.score_detail;
    Ok(PlayResultView {
        score: result.score,
        round_score: state.round_score,
        round_target: state.round_target,
        round_over: state.is_round_over(),
        fu: detail.fu,
        fan: detail.fan,
        mult: detail.mult,
        patterns: result.patterns_matched,
        breakdown: detail.breakdown.clone(),
    })
}

#[tauri::command]
pub fn end_round(state: State<'_, Mutex<GameState>>) -> Result<EndRoundView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let outcome = state.end_round();
    let view = match outcome {
        crate::game::state::RoundOutcome::Pass { score, target, bonus } => EndRoundView {
            passed: true,
            score,
            target,
            bonus,
            victory: false,
        },
        crate::game::state::RoundOutcome::Fail { score, target } => EndRoundView {
            passed: false,
            score,
            target,
            bonus: 0,
            victory: false,
        },
        crate::game::state::RoundOutcome::Victory => EndRoundView {
            passed: true,
            score: state.round_score,
            target: state.round_target,
            bonus: 0,
            victory: true,
        },
    };
    Ok(view)
}

#[derive(serde::Serialize)]
pub struct EndRoundView {
    pub passed: bool,
    pub score: u64,
    pub target: u64,
    pub bonus: u32,
    pub victory: bool,
}

impl From<&GameState> for GameStateView {
    fn from(state: &GameState) -> Self {
        let mut tiles: Vec<TileView> = state.hand_tiles.iter().map(TileView::from).collect();
        tiles.sort_by(|a, b| {
            let sa = suit_sort_key(&a.suit, a.rank);
            let sb = suit_sort_key(&b.suit, b.rank);
            sa.cmp(&sb)
        });
        Self {
            phase: format!("{:?}", state.phase),
            current_wind: format!("{:?}", state.current_wind),
            current_round: state.current_round,
            current_play: state.current_play,
            max_plays: state.max_plays,
            hand_tiles: tiles,
            wall_remaining: state.wall.remaining(),
            round_score: state.round_score,
            round_target: state.round_target,
            currency: state.currency,
            artifacts: state.artifacts.iter().map(ArtifactView::from).collect(),
        }
    }
}

fn suit_sort_key(suit: &str, rank: u8) -> (u8, u8) {
    let order = match suit {
        "Manzu" => 0,
        "Pinzu" => 1,
        "Souzu" => 2,
        "Wind" => 3,
        "Dragon" => 4,
        _ => 5,
    };
    (order, rank)
}

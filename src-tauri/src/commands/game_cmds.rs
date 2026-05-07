use std::sync::Mutex;
use tauri::State;

use crate::game::state::GameState;
use crate::models::tile::Tile;

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
    pub artifact_count: usize,
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
}

#[tauri::command]
pub fn submit_play(state: State<'_, Mutex<GameState>>) -> Result<PlayResultView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let result = state.submit_play()?;
    Ok(PlayResultView {
        score: result.score,
        round_score: state.round_score,
        round_target: state.round_target,
        round_over: state.is_round_over(),
    })
}

impl From<&GameState> for GameStateView {
    fn from(state: &GameState) -> Self {
        Self {
            phase: format!("{:?}", state.phase),
            current_wind: format!("{:?}", state.current_wind),
            current_round: state.current_round,
            current_play: state.current_play,
            max_plays: state.max_plays,
            hand_tiles: state.hand_tiles.iter().map(TileView::from).collect(),
            wall_remaining: state.wall.remaining(),
            round_score: state.round_score,
            round_target: state.round_target,
            currency: state.currency,
            artifact_count: state.artifacts.len(),
        }
    }
}

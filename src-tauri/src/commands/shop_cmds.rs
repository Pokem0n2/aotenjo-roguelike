use std::sync::Mutex;
use tauri::State;

use crate::game::state::GameState;
use super::game_cmds::GameStateView;

#[tauri::command]
pub fn shop_buy(index: usize, state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.shop_buy(index)?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn shop_sell_artifact(artifact_id: String, state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.shop_sell_artifact(&artifact_id)?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn shop_sell_gadget(gadget_id: String, state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.shop_sell_gadget(&gadget_id)?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn shop_reroll(state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.shop_reroll()?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn shop_leave(state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.shop_leave();
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn use_gadget(
    gadget_id: String,
    tile_ids: Vec<u32>,
    state: State<'_, Mutex<GameState>>,
) -> Result<GadgetUseResult, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let message = state.use_gadget(&gadget_id, tile_ids)?;
    Ok(GadgetUseResult {
        message,
        game_state: GameStateView::from(&*state),
    })
}

#[derive(serde::Serialize)]
pub struct GadgetUseResult {
    pub message: String,
    pub game_state: GameStateView,
}

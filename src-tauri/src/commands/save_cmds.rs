use std::sync::Mutex;
use tauri::State;

use crate::game::state::GameState;
use crate::persistence::SaveMeta;
use super::game_cmds::GameStateView;

#[derive(serde::Serialize)]
pub struct SaveResult {
    pub success: bool,
    pub message: String,
    pub meta: Option<SaveMeta>,
}

#[tauri::command]
pub fn save_game(slot: usize, state: State<'_, Mutex<GameState>>) -> Result<SaveResult, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let meta = state.save_to_slot(slot)?;
    Ok(SaveResult {
        success: true,
        message: format!("存档 {} 保存成功", slot + 1),
        meta: Some(meta),
    })
}

#[tauri::command]
pub fn load_game(slot: usize, state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let (loaded, _meta) = GameState::load_from_slot(slot)?;
    let mut state = state.lock().map_err(|e| e.to_string())?;
    *state = loaded;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn list_saves() -> Vec<SaveMeta> {
    GameState::list_saves()
}

#[tauri::command]
pub fn delete_save(slot: usize) -> Result<String, String> {
    GameState::delete_save(slot)?;
    Ok(format!("存档 {} 已删除", slot + 1))
}

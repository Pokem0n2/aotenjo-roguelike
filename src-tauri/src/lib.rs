use std::sync::Mutex;

mod models;
mod game;
mod commands;
mod persistence;

use game::state::GameState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(GameState::new()))
        .invoke_handler(tauri::generate_handler![
            commands::game_cmds::start_run,
            commands::game_cmds::draw_tiles,
            commands::game_cmds::get_game_state,
            commands::game_cmds::select_tiles_for_play,
            commands::game_cmds::submit_play,
            commands::game_cmds::skip_play,
            commands::game_cmds::end_round,
            commands::game_cmds::start_next_round,
            commands::shop_cmds::shop_buy,
            commands::shop_cmds::shop_sell_artifact,
            commands::shop_cmds::shop_sell_gadget,
            commands::shop_cmds::shop_reroll,
            commands::shop_cmds::shop_leave,
            commands::shop_cmds::use_gadget,
            commands::save_cmds::save_game,
            commands::save_cmds::load_game,
            commands::save_cmds::list_saves,
            commands::save_cmds::delete_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

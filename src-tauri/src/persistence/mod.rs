use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::game::state::GameState;

const MAX_SAVE_SLOTS: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveMeta {
    pub slot: usize,
    pub wind: String,
    pub round: u8,
    pub currency: u32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    checksum: u64,
    data: GameState,
}

impl GameState {
    pub fn save_to_slot(&self, slot: usize) -> Result<SaveMeta, String> {
        if slot >= MAX_SAVE_SLOTS {
            return Err(format!("存档位 {} 无效 (0-{})", slot, MAX_SAVE_SLOTS - 1));
        }

        let dir = save_dir()?;
        fs::create_dir_all(&dir).map_err(|e| format!("创建存档目录失败: {}", e))?;

        let json = serde_json::to_string(self)
            .map_err(|e| format!("序列化失败: {}", e))?;

        let checksum = simple_hash(&json);
        let save = SaveFile {
            checksum,
            data: self.clone(),
        };

        let save_json = serde_json::to_string(&save)
            .map_err(|e| format!("序列化存档失败: {}", e))?;

        let path = save_dir()?.join(format!("slot_{}.json", slot));
        fs::write(&path, save_json)
            .map_err(|e| format!("写入存档失败: {}", e))?;

        Ok(SaveMeta {
            slot,
            wind: format!("{:?}", self.current_wind),
            round: self.current_round,
            currency: self.currency,
            timestamp: chrono_now(),
        })
    }

    pub fn load_from_slot(slot: usize) -> Result<(GameState, SaveMeta), String> {
        if slot >= MAX_SAVE_SLOTS {
            return Err(format!("存档位 {} 无效", slot));
        }

        let path = save_dir()?.join(format!("slot_{}.json", slot));
        if !path.exists() {
            return Err("该存档位为空".to_string());
        }

        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("读取存档失败: {}", e))?;

        let save: SaveFile = serde_json::from_str(&file_content)
            .map_err(|e| format!("解析存档失败: {}", e))?;

        // Verify checksum
        let data_json = serde_json::to_string(&save.data)
            .map_err(|e| format!("重新序列化失败: {}", e))?;
        let computed = simple_hash(&data_json);
        if computed != save.checksum {
            return Err("存档校验失败 — 文件可能已损坏".to_string());
        }

        let meta = SaveMeta {
            slot,
            wind: format!("{:?}", save.data.current_wind),
            round: save.data.current_round,
            currency: save.data.currency,
            timestamp: chrono_now(),
        };

        Ok((save.data, meta))
    }

    pub fn list_saves() -> Vec<SaveMeta> {
        let mut saves = Vec::new();
        for slot in 0..MAX_SAVE_SLOTS {
            let path = match save_dir() {
                Ok(dir) => dir.join(format!("slot_{}.json", slot)),
                Err(_) => continue,
            };
            if path.exists() {
                if let Ok(file_content) = fs::read_to_string(&path) {
                    if let Ok(save) = serde_json::from_str::<SaveFile>(&file_content) {
                        saves.push(SaveMeta {
                            slot,
                            wind: format!("{:?}", save.data.current_wind),
                            round: save.data.current_round,
                            currency: save.data.currency,
                            timestamp: chrono_now(),
                        });
                    }
                }
            }
        }
        saves
    }

    pub fn delete_save(slot: usize) -> Result<(), String> {
        let path = save_dir()?.join(format!("slot_{}.json", slot));
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("删除存档失败: {}", e))?;
        }
        Ok(())
    }
}

fn save_dir() -> Result<PathBuf, String> {
    let app_data = dirs_sys::known_folder(&dirs_sys::FOLDERID_RoamingAppData)
        .unwrap_or_else(|| PathBuf::from("."));
    Ok(app_data.join("aotenjo-roguelike").join("saves"))
}

/// Simple FNV-1a hash for checksum
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", now)
}

// Minimal dirs_sys for Windows APPDATA
mod dirs_sys {
    use std::path::PathBuf;

    pub const FOLDERID_RoamingAppData: &str = "RoamingAppData";

    pub fn known_folder(_id: &&str) -> Option<PathBuf> {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    }
}

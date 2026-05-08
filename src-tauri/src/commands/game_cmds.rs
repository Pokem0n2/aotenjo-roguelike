use std::sync::Mutex;
use tauri::State;

use crate::game::state::GameState;
use crate::game::shop::ShopItemType;
use crate::models::tile::Tile;
use crate::models::artifact::Artifact;

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
    pub gadgets: Vec<GadgetView>,
    pub boss: Option<BossView>,
    pub tax_per_play: u32,
    pub boss_disabled: bool,
    pub shop_items: Vec<ShopItemView>,
    pub shop_rerolls: u8,
    pub skip_count: u8,
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

#[derive(serde::Serialize)]
pub struct GadgetView {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub description_zh: String,
}

impl From<&crate::models::gadget::Gadget> for GadgetView {
    fn from(gadget: &crate::models::gadget::Gadget) -> Self {
        Self {
            id: gadget.id.clone(),
            name_zh: gadget.name_zh.clone(),
            name_en: gadget.name_en.clone(),
            description_zh: gadget.description_zh.clone(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct BossView {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub gimmick_description: String,
    pub score_multiplier: f64,
}

impl From<&crate::models::boss::Boss> for BossView {
    fn from(boss: &crate::models::boss::Boss) -> Self {
        let gimmick_description = match &boss.gimmick {
            crate::models::boss::BossGimmick::BlindSuit(suit) => {
                let suit_name = match suit {
                    crate::models::tile::TileSuit::Manzu => "万子",
                    crate::models::tile::TileSuit::Pinzu => "筒子",
                    crate::models::tile::TileSuit::Souzu => "索子",
                    _ => "未知",
                };
                format!("隐藏所有{}牌面", suit_name)
            }
            crate::models::boss::BossGimmick::ReducedPlays(n) => format!("出牌次数减少为{}", n),
            crate::models::boss::BossGimmick::ReducedSkipBonus(n) => format!("跳过只摸{}张牌", n),
            crate::models::boss::BossGimmick::TaxPerPlay(t) => format!("每次出牌缴纳{}金币", t),
            crate::models::boss::BossGimmick::ScoreThresholdScale(s) => format!("目标分数×{:.1}", s),
            crate::models::boss::BossGimmick::ForcedPattern(p) => format!("完成特定牌型可获得奖励番"),
        };
        Self {
            id: boss.id.clone(),
            name_zh: boss.name_zh.clone(),
            name_en: boss.name_en.clone(),
            gimmick_description,
            score_multiplier: boss.score_multiplier,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ShopItemView {
    pub index: usize,
    pub item_type: String,
    pub cost: u32,
    pub sold: bool,
    pub name_zh: String,
    pub description_zh: String,
    pub rarity: Option<String>,
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

#[tauri::command]
pub fn skip_play(state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.skip_play()?;
    Ok(GameStateView::from(&*state))
}

#[tauri::command]
pub fn start_next_round(state: State<'_, Mutex<GameState>>) -> Result<GameStateView, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.start_next_round();
    Ok(GameStateView::from(&*state))
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

        let shop_items: Vec<ShopItemView> = state.shop_items.iter().enumerate().map(|(i, item)| {
            match &item.item_type {
                ShopItemType::Artifact(a) => ShopItemView {
                    index: i,
                    item_type: "artifact".to_string(),
                    cost: item.cost,
                    sold: item.sold,
                    name_zh: a.name_zh.clone(),
                    description_zh: a.description_zh.clone(),
                    rarity: Some(format!("{:?}", a.rarity)),
                },
                ShopItemType::Gadget(g) => ShopItemView {
                    index: i,
                    item_type: "gadget".to_string(),
                    cost: item.cost,
                    sold: item.sold,
                    name_zh: g.name_zh.clone(),
                    description_zh: g.description_zh.clone(),
                    rarity: None,
                },
            }
        }).collect();

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
            gadgets: state.gadgets.iter().map(GadgetView::from).collect(),
            boss: state.boss.as_ref().map(BossView::from),
            tax_per_play: state.tax_per_play,
            boss_disabled: state.boss_disabled,
            shop_items,
            shop_rerolls: state.shop_rerolls,
            skip_count: state.skip_count,
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

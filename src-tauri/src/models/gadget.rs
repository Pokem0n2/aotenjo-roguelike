use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gadget {
    pub id: String,
    pub name_zh: String,
    pub name_en: String,
    pub description_zh: String,
    pub effect: GadgetEffect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GadgetEffect {
    DestroyTiles { count: usize },
    SwapTiles,
    PeekWall { count: usize },
    TransformTile,
    BuffTile { fu_bonus: i32, mult_bonus: i32 },
}

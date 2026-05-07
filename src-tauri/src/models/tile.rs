use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileSuit {
    Manzu,  // 万子 (Characters)
    Pinzu,  // 筒子 (Circles)
    Souzu,  // 索子 (Bamboo)
    Wind,   // 风牌
    Dragon, // 三元牌
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wind {
    East,  // 东
    South, // 南
    West,  // 西
    North, // 北
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dragon {
    White,  // 白板
    Green,  // 發
    Red,    // 中
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub suit: TileSuit,
    pub rank: u8,
    pub id: u32,
    pub is_red: bool,
    pub buff_fu: i32,
    pub buff_mult: i32,
}

impl Tile {
    pub fn new(suit: TileSuit, rank: u8, id: u32) -> Self {
        Self {
            suit,
            rank,
            id,
            is_red: false,
            buff_fu: 0,
            buff_mult: 0,
        }
    }

    pub fn with_red(mut self) -> Self {
        self.is_red = true;
        self
    }

    /// Intrinsic Fu value for scoring
    pub fn base_fu(&self) -> u64 {
        match self.suit {
            TileSuit::Manzu | TileSuit::Pinzu | TileSuit::Souzu => {
                if self.rank == 1 || self.rank == 9 {
                    6 // Terminal tiles
                } else {
                    3 // Simple tiles (2-8)
                }
            }
            TileSuit::Wind => 8,
            TileSuit::Dragon => 10,
        }
    }

    pub fn display_zh(&self) -> String {
        match self.suit {
            TileSuit::Manzu => {
                let chars = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
                format!("{}万", chars[(self.rank - 1) as usize])
            }
            TileSuit::Pinzu => {
                let chars = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
                format!("{}筒", chars[(self.rank - 1) as usize])
            }
            TileSuit::Souzu => {
                let chars = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
                format!("{}索", chars[(self.rank - 1) as usize])
            }
            TileSuit::Wind => {
                let chars = ["东", "南", "西", "北"];
                chars[(self.rank - 1) as usize].to_string()
            }
            TileSuit::Dragon => {
                let chars = ["白", "發", "中"];
                chars[(self.rank - 1) as usize].to_string()
            }
        }
    }

    pub fn display_short(&self) -> String {
        match self.suit {
            TileSuit::Manzu => format!("{}m", self.rank),
            TileSuit::Pinzu => format!("{}p", self.rank),
            TileSuit::Souzu => format!("{}s", self.rank),
            TileSuit::Wind => {
                let chars = ["E", "S", "W", "N"];
                chars[(self.rank - 1) as usize].to_string()
            }
            TileSuit::Dragon => {
                let chars = ["Wh", "G", "R"];
                chars[(self.rank - 1) as usize].to_string()
            }
        }
    }

    /// Is this a terminal (1 or 9) or honor (wind/dragon) tile?
    pub fn is_terminal_or_honor(&self) -> bool {
        match self.suit {
            TileSuit::Manzu | TileSuit::Pinzu | TileSuit::Souzu => {
                self.rank == 1 || self.rank == 9
            }
            TileSuit::Wind | TileSuit::Dragon => true,
        }
    }

    /// Is this a simple tile (2-8 in a numbered suit)?
    pub fn is_simple(&self) -> bool {
        matches!(self.suit, TileSuit::Manzu | TileSuit::Pinzu | TileSuit::Souzu)
            && (2..=8).contains(&self.rank)
    }

    /// Same tile type (ignoring id and buffs)?
    pub fn same_type(&self, other: &Tile) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

/// Generate the standard 136-tile set (34 types × 4 copies, with red dora replacements)
pub fn standard_tile_set() -> Vec<Tile> {
    let mut tiles = Vec::with_capacity(136);
    let mut id: u32 = 0;

    let suits = [
        TileSuit::Manzu,
        TileSuit::Pinzu,
        TileSuit::Souzu,
    ];

    for suit in suits {
        for rank in 1u8..=9u8 {
            for copy in 0u8..4 {
                let tile = if rank == 5 && copy == 0 {
                    Tile::new(suit, rank, id).with_red()
                } else {
                    Tile::new(suit, rank, id)
                };
                tiles.push(tile);
                id += 1;
            }
        }
    }

    // Wind tiles: 4 winds × 4 copies = 16
    for rank in 1u8..=4u8 {
        for _ in 0u8..4 {
            tiles.push(Tile::new(TileSuit::Wind, rank, id));
            id += 1;
        }
    }

    // Dragon tiles: 3 dragons × 4 copies = 12
    for rank in 1u8..=3u8 {
        for _ in 0u8..4 {
            tiles.push(Tile::new(TileSuit::Dragon, rank, id));
            id += 1;
        }
    }

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_set_count() {
        let tiles = standard_tile_set();
        assert_eq!(tiles.len(), 136);
    }

    #[test]
    fn test_red_dora_count() {
        let tiles = standard_tile_set();
        let red_count = tiles.iter().filter(|t| t.is_red).count();
        assert_eq!(red_count, 3); // 5m, 5p, 5s
    }

    #[test]
    fn test_tile_type_counts() {
        let tiles = standard_tile_set();
        let mut counts = std::collections::HashMap::new();
        for t in &tiles {
            let key = (t.suit, t.rank);
            *counts.entry(key).or_insert(0) += 1;
        }
        assert_eq!(counts.len(), 34);
        for count in counts.values() {
            assert_eq!(*count, 4);
        }
    }

    #[test]
    fn test_base_fu() {
        assert_eq!(Tile::new(TileSuit::Manzu, 5, 0).base_fu(), 3);
        assert_eq!(Tile::new(TileSuit::Manzu, 1, 0).base_fu(), 6);
        assert_eq!(Tile::new(TileSuit::Manzu, 9, 0).base_fu(), 6);
        assert_eq!(Tile::new(TileSuit::Wind, 1, 0).base_fu(), 8);
        assert_eq!(Tile::new(TileSuit::Dragon, 2, 0).base_fu(), 10);
    }

    #[test]
    fn test_terminal_or_honor() {
        assert!(Tile::new(TileSuit::Manzu, 1, 0).is_terminal_or_honor());
        assert!(Tile::new(TileSuit::Manzu, 9, 0).is_terminal_or_honor());
        assert!(!Tile::new(TileSuit::Manzu, 5, 0).is_terminal_or_honor());
        assert!(Tile::new(TileSuit::Wind, 1, 0).is_terminal_or_honor());
        assert!(Tile::new(TileSuit::Dragon, 1, 0).is_terminal_or_honor());
    }
}

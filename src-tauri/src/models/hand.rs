use serde::{Deserialize, Serialize};

use super::tile::Tile;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeldKind {
    Chi,  // 顺子 (Sequence: 3 consecutive tiles in same suit)
    Pon,  // 刻子 (Triplet: 3 identical tiles)
    Kan,  // 杠子 (Quad: 4 identical tiles)
    Pair, // 对子 (Pair: 2 identical tiles)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meld {
    pub kind: MeldKind,
    pub tiles: Vec<Tile>,
}

impl Meld {
    pub fn new(kind: MeldKind, tiles: Vec<Tile>) -> Self {
        Self { kind, tiles }
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
}

/// A single play submission (one of the 4 plays per round)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Play {
    pub melds: Vec<Meld>,
    pub pair: Option<Meld>,
}

impl Play {
    pub fn new(melds: Vec<Meld>, pair: Option<Meld>) -> Self {
        Self { melds, pair }
    }

    pub fn all_tiles(&self) -> Vec<&Tile> {
        let mut tiles: Vec<&Tile> = Vec::new();
        for meld in &self.melds {
            tiles.extend(meld.tiles.iter());
        }
        if let Some(ref pair) = self.pair {
            tiles.extend(pair.tiles.iter());
        }
        tiles
    }

    pub fn total_tile_count(&self) -> usize {
        let mut count = 0;
        for meld in &self.melds {
            count += meld.tile_count();
        }
        if let Some(ref pair) = self.pair {
            count += pair.tile_count();
        }
        count
    }
}

/// Validate whether selected tiles form a valid play structure:
/// - One meld (3 or 4 tiles) + one pair (2 tiles)
///   = 5 tiles (chi/pon + pair) or 6 tiles (kan + pair)
pub fn validate_play_structure(tiles: &[Tile]) -> Result<Play, String> {
    if tiles.len() < 5 || tiles.len() > 14 {
        return Err(format!("选牌数量无效: {}张 (需要5-14张)", tiles.len()));
    }

    // For the basic play, expect 5 or 6 tiles (one meld + one pair)
    if tiles.len() == 5 || tiles.len() == 6 {
        return validate_single_meld_play(tiles);
    }

    // For a full hand (14 tiles), validate standard 4-meld + pair structure
    validate_full_hand(tiles)
}

fn validate_single_meld_play(tiles: &[Tile]) -> Result<Play, String> {
    let n = tiles.len();

    // Try to find a valid meld + pair combination
    if n == 5 {
        // 3-tile meld + 2-tile pair
        for i in 0..tiles.len() {
            for j in i + 1..tiles.len() {
                let pair_tiles = vec![tiles[i], tiles[j]];
                if is_pair(&pair_tiles) {
                    let remaining: Vec<Tile> = tiles
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| *idx != i && *idx != j)
                        .map(|(_, t)| *t)
                        .collect();

                    if let Some(kind) = is_meld(&remaining) {
                        return Ok(Play::new(
                            vec![Meld::new(kind, remaining)],
                            Some(Meld::new(MeldKind::Pair, pair_tiles)),
                        ));
                    }
                }
            }
        }
    } else if n == 6 {
        // 4-tile kan + 2-tile pair
        for i in 0..tiles.len() {
            for j in i + 1..tiles.len() {
                let pair_tiles = vec![tiles[i], tiles[j]];
                if is_pair(&pair_tiles) {
                    let remaining: Vec<Tile> = tiles
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| *idx != i && *idx != j)
                        .map(|(_, t)| *t)
                        .collect();

                    if remaining.len() == 4 && is_kan(&remaining) {
                        return Ok(Play::new(
                            vec![Meld::new(MeldKind::Kan, remaining)],
                            Some(Meld::new(MeldKind::Pair, pair_tiles)),
                        ));
                    }
                }
            }
        }

        // Also try 3-tile meld + pair, with 1 extra (shouldn't be valid for 6 tiles)
    }

    Err("选中的牌无法组成合法的面子+对子组合".to_string())
}

fn validate_full_hand(tiles: &[Tile]) -> Result<Play, String> {
    // Standard mahjong hand: 4 melds + 1 pair = 14 tiles
    // This is used for special hands like Seven Pairs, Thirteen Orphans
    if tiles.len() == 14 {
        // Try standard decomposition
        if let Some(result) = try_standard_decomposition(tiles) {
            return Ok(result);
        }

        // Try Seven Pairs
        if is_seven_pairs(tiles) {
            let pairs: Vec<Meld> = tiles
                .chunks(2)
                .map(|chunk| Meld::new(MeldKind::Pair, chunk.to_vec()))
                .collect();
            return Ok(Play::new(pairs, None));
        }

        // Try Thirteen Orphans
        if is_thirteen_orphans(tiles) {
            // Find the pair
            let mut pair_tile_rank = None;
            for tile in tiles {
                let count = tiles.iter().filter(|t| t.same_type(tile)).count();
                if count == 2 {
                    pair_tile_rank = Some((tile.suit, tile.rank));
                    break;
                }
            }
            return Ok(Play::new(vec![], None)); // Special hand, no melds
        }
    }

    Err("无法组成合法的和牌牌型".to_string())
}

fn try_standard_decomposition(tiles: &[Tile]) -> Option<Play> {
    // Build frequency map
    let mut freq: std::collections::HashMap<(super::tile::TileSuit, u8), Vec<usize>> =
        std::collections::HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        freq.entry((tile.suit, tile.rank))
            .or_default()
            .push(i);
    }

    // Try each possible pair
    let pair_keys: Vec<_> = freq
        .iter()
        .filter(|(_, indices)| indices.len() >= 2)
        .map(|(key, _)| *key)
        .collect();

    for pair_key in &pair_keys {
        let mut remaining = tiles.to_vec();
        // Remove pair tiles
        let mut removed = 0;
        remaining.retain(|t| {
            if (t.suit, t.rank) == *pair_key && removed < 2 {
                removed += 1;
                false
            } else {
                true
            }
        });

        if remaining.len() == 12 {
            if let Some(melds) = try_decompose_melds(&remaining) {
                let pair_tiles: Vec<Tile> = tiles
                    .iter()
                    .filter(|t| (t.suit, t.rank) == *pair_key)
                    .take(2)
                    .copied()
                    .collect();
                return Some(Play::new(
                    melds,
                    Some(Meld::new(MeldKind::Pair, pair_tiles)),
                ));
            }
        }
    }

    None
}

fn try_decompose_melds(tiles: &[Tile]) -> Option<Vec<Meld>> {
    if tiles.is_empty() {
        return Some(vec![]);
    }

    // Try to find a meld starting from the first tile
    // Try Pon (triplet) first
    let first = &tiles[0];
    let same_count = tiles.iter().filter(|t| t.same_type(first)).count();

    if same_count >= 3 {
        let mut remaining: Vec<Tile> = tiles.to_vec();
        let meld_tiles: Vec<Tile> = {
            let mut meld = Vec::new();
            let mut count = 0;
            remaining.retain(|t| {
                if t.same_type(first) && count < 3 {
                    meld.push(*t);
                    count += 1;
                    false
                } else {
                    true
                }
            });
            meld
        };
        if let Some(mut rest) = try_decompose_melds(&remaining) {
            let mut result = vec![Meld::new(MeldKind::Pon, meld_tiles)];
            result.append(&mut rest);
            return Some(result);
        }
    }

    // Try Chi (sequence) for numbered suits
    if matches!(
        first.suit,
        super::tile::TileSuit::Manzu | super::tile::TileSuit::Pinzu | super::tile::TileSuit::Souzu
    ) && first.rank <= 7
    {
        if let Some(seq) = try_find_chi(tiles, first.suit, first.rank) {
            let seq_ids: std::collections::HashSet<u32> =
                seq.iter().map(|t| t.id).collect();
            let remaining: Vec<Tile> =
                tiles.iter().filter(|t| !seq_ids.contains(&t.id)).copied().collect();
            if let Some(mut rest) = try_decompose_melds(&remaining) {
                let mut result = vec![Meld::new(MeldKind::Chi, seq)];
                result.append(&mut rest);
                return Some(result);
            }
        }
    }

    None
}

fn try_find_chi(tiles: &[Tile], suit: super::tile::TileSuit, start_rank: u8) -> Option<Vec<Tile>> {
    let mut result = Vec::new();
    for rank in start_rank..start_rank + 3 {
        let tile = tiles.iter().find(|t| t.suit == suit && t.rank == rank);
        if let Some(t) = tile {
            result.push(*t);
        } else {
            return None;
        }
    }
    Some(result)
}

pub fn is_pair(tiles: &[Tile]) -> bool {
    tiles.len() == 2 && tiles[0].same_type(&tiles[1])
}

pub fn is_meld(tiles: &[Tile]) -> Option<MeldKind> {
    if tiles.len() == 3 {
        if is_pon(tiles) {
            return Some(MeldKind::Pon);
        }
        if is_chi(tiles) {
            return Some(MeldKind::Chi);
        }
    }
    if tiles.len() == 4 && is_kan(tiles) {
        return Some(MeldKind::Kan);
    }
    None
}

pub fn is_pon(tiles: &[Tile]) -> bool {
    tiles.len() == 3 && tiles[0].same_type(&tiles[1]) && tiles[1].same_type(&tiles[2])
}

pub fn is_chi(tiles: &[Tile]) -> bool {
    if tiles.len() != 3 {
        return false;
    }
    let suit = tiles[0].suit;
    if !matches!(
        suit,
        super::tile::TileSuit::Manzu | super::tile::TileSuit::Pinzu | super::tile::TileSuit::Souzu
    ) {
        return false;
    }
    if tiles[1].suit != suit || tiles[2].suit != suit {
        return false;
    }
    let mut ranks: Vec<u8> = tiles.iter().map(|t| t.rank).collect();
    ranks.sort();
    ranks[0] + 1 == ranks[1] && ranks[1] + 1 == ranks[2]
}

pub fn is_kan(tiles: &[Tile]) -> bool {
    tiles.len() == 4
        && tiles[0].same_type(&tiles[1])
        && tiles[1].same_type(&tiles[2])
        && tiles[2].same_type(&tiles[3])
}

fn is_seven_pairs(tiles: &[Tile]) -> bool {
    if tiles.len() != 14 {
        return false;
    }
    let mut freq: std::collections::HashMap<(super::tile::TileSuit, u8), usize> =
        std::collections::HashMap::new();
    for t in tiles {
        *freq.entry((t.suit, t.rank)).or_insert(0) += 1;
    }
    freq.values().all(|&c| c == 2) && freq.len() == 7
}

fn is_thirteen_orphans(tiles: &[Tile]) -> bool {
    if tiles.len() != 14 {
        return false;
    }
    // Must have one of each terminal/honor + one duplicate
    let required: std::collections::HashSet<(super::tile::TileSuit, u8)> = [
        (super::tile::TileSuit::Manzu, 1),
        (super::tile::TileSuit::Manzu, 9),
        (super::tile::TileSuit::Pinzu, 1),
        (super::tile::TileSuit::Pinzu, 9),
        (super::tile::TileSuit::Souzu, 1),
        (super::tile::TileSuit::Souzu, 9),
        (super::tile::TileSuit::Wind, 1),
        (super::tile::TileSuit::Wind, 2),
        (super::tile::TileSuit::Wind, 3),
        (super::tile::TileSuit::Wind, 4),
        (super::tile::TileSuit::Dragon, 1),
        (super::tile::TileSuit::Dragon, 2),
        (super::tile::TileSuit::Dragon, 3),
    ]
    .iter()
    .copied()
    .collect();

    let tile_types: std::collections::HashSet<(super::tile::TileSuit, u8)> =
        tiles.iter().map(|t| (t.suit, t.rank)).collect();

    if tile_types.len() != 13 {
        return false;
    }

    required.iter().all(|r| tile_types.contains(r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tile::{TileSuit, Tile};

    #[test]
    fn test_is_pair() {
        let tiles = vec![Tile::new(TileSuit::Manzu, 1, 0), Tile::new(TileSuit::Manzu, 1, 1)];
        assert!(is_pair(&tiles));
    }

    #[test]
    fn test_is_pon() {
        let tiles = vec![
            Tile::new(TileSuit::Manzu, 3, 0),
            Tile::new(TileSuit::Manzu, 3, 1),
            Tile::new(TileSuit::Manzu, 3, 2),
        ];
        assert!(is_pon(&tiles));
        assert!(is_meld(&tiles).is_some());
    }

    #[test]
    fn test_is_chi() {
        let tiles = vec![
            Tile::new(TileSuit::Pinzu, 2, 0),
            Tile::new(TileSuit::Pinzu, 3, 1),
            Tile::new(TileSuit::Pinzu, 4, 2),
        ];
        assert!(is_chi(&tiles));
        assert!(is_meld(&tiles).is_some());
    }

    #[test]
    fn test_is_kan() {
        let tiles = vec![
            Tile::new(TileSuit::Souzu, 7, 0),
            Tile::new(TileSuit::Souzu, 7, 1),
            Tile::new(TileSuit::Souzu, 7, 2),
            Tile::new(TileSuit::Souzu, 7, 3),
        ];
        assert!(is_kan(&tiles));
        assert_eq!(is_meld(&tiles), Some(MeldKind::Kan));
    }

    #[test]
    fn test_validate_play_chi_pair() {
        let tiles = vec![
            Tile::new(TileSuit::Manzu, 1, 0),
            Tile::new(TileSuit::Manzu, 2, 1),
            Tile::new(TileSuit::Manzu, 3, 2),
            Tile::new(TileSuit::Pinzu, 5, 3),
            Tile::new(TileSuit::Pinzu, 5, 4),
        ];
        let play = validate_play_structure(&tiles);
        assert!(play.is_ok());
    }

    #[test]
    fn test_validate_play_pon_pair() {
        let tiles = vec![
            Tile::new(TileSuit::Dragon, 3, 0), // 中
            Tile::new(TileSuit::Dragon, 3, 1),
            Tile::new(TileSuit::Dragon, 3, 2),
            Tile::new(TileSuit::Wind, 1, 3), // 东
            Tile::new(TileSuit::Wind, 1, 4),
        ];
        let play = validate_play_structure(&tiles);
        assert!(play.is_ok());
    }

    #[test]
    fn test_validate_invalid_play() {
        let tiles = vec![
            Tile::new(TileSuit::Manzu, 1, 0),
            Tile::new(TileSuit::Manzu, 3, 1),
            Tile::new(TileSuit::Manzu, 5, 2),
            Tile::new(TileSuit::Pinzu, 7, 3),
            Tile::new(TileSuit::Souzu, 9, 4),
        ];
        let play = validate_play_structure(&tiles);
        assert!(play.is_err());
    }
}

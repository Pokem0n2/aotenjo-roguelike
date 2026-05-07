use crate::models::hand::MeldKind;
use crate::models::tile::{TileSuit, Tile};
use crate::models::pattern::PatternMatch;
use crate::models::artifact::Rarity;
use std::collections::HashMap;

/// Check all patterns against a play's tiles.
/// `play_tiles`: the tiles in the current play (meld + pair)
/// `hand_tiles`: all tiles currently in the player's hand (for context)
/// `all_played_tiles`: all tiles played so far this round (for cumulative checks)
pub fn check_patterns(
    play_tiles: &[Tile],
    all_played_tiles: &[Tile],
    _hand_tiles: &[Tile],
) -> Vec<PatternMatch> {
    let mut matches = Vec::new();

    // Build frequency map of play tiles
    let freq = build_freq(play_tiles);

    // Check each pattern
    check_common_patterns(&freq, play_tiles, &mut matches);
    check_suit_patterns(&freq, play_tiles, &mut matches);
    check_honor_patterns(&freq, play_tiles, &mut matches);
    check_special_patterns(play_tiles, all_played_tiles, &mut matches);

    // Deduplicate and sort by fan value (highest first)
    matches.sort_by(|a, b| b.base_fan.partial_cmp(&a.base_fan).unwrap_or(std::cmp::Ordering::Equal));
    matches.dedup_by(|a, b| a.pattern_id == b.pattern_id);

    matches
}

fn build_freq(tiles: &[Tile]) -> HashMap<(TileSuit, u8), usize> {
    let mut freq = HashMap::new();
    for tile in tiles {
        *freq.entry((tile.suit, tile.rank)).or_insert(0) += 1;
    }
    freq
}

fn check_common_patterns(
    freq: &HashMap<(TileSuit, u8), usize>,
    tiles: &[Tile],
    matches: &mut Vec<PatternMatch>,
) {
    // 断么九 (All Simples) - no terminals or honors
    if tiles.iter().all(|t| t.is_simple()) {
        matches.push(PatternMatch {
            pattern_id: "tanyao".into(),
            name_zh: "断么九".into(),
            base_fan: 1.0,
            rarity: Rarity::Common,
        });
    }

    // 全带么 (All Terminals and Honors) - every meld contains at least one terminal/honor
    if !tiles.is_empty() && tiles.iter().all(|t| t.is_terminal_or_honor()) {
        matches.push(PatternMatch {
            pattern_id: "chanta".into(),
            name_zh: "全带么".into(),
            base_fan: 2.0,
            rarity: Rarity::Uncommon,
        });
    }

    // 对对和 (All Triplets) - all melds are pon/kan, no chi
    let all_triplets = tiles.len() >= 5 && {
        let mut freq_clone = freq.clone();
        let mut has_chi = false;
        for (&(suit, rank), &count) in freq.iter() {
            if count >= 3 {
                *freq_clone.get_mut(&(suit, rank)).unwrap() -= 3;
            }
        }
        // Check if remaining can form sequences
        for (&(suit, rank), &count) in &freq_clone {
            if count > 0 && is_numbered_suit(suit) {
                if rank <= 7 {
                    let c1 = freq_clone.get(&(suit, rank)).copied().unwrap_or(0);
                    let c2 = freq_clone.get(&(suit, rank + 1)).copied().unwrap_or(0);
                    let c3 = freq_clone.get(&(suit, rank + 2)).copied().unwrap_or(0);
                    if c1 > 0 && c2 > 0 && c3 > 0 {
                        has_chi = true;
                    }
                }
            }
        }
        !has_chi && freq.values().any(|&c| c >= 3)
    };
    if all_triplets {
        matches.push(PatternMatch {
            pattern_id: "toitoi".into(),
            name_zh: "对对和".into(),
            base_fan: 2.0,
            rarity: Rarity::Uncommon,
        });
    }

    // 三色同顺 (Mixed Triple Chow) - same sequence in all three suits
    for rank in 1u8..=7u8 {
        let has_m = freq.contains_key(&(TileSuit::Manzu, rank))
            && freq.contains_key(&(TileSuit::Manzu, rank + 1))
            && freq.contains_key(&(TileSuit::Manzu, rank + 2));
        let has_p = freq.contains_key(&(TileSuit::Pinzu, rank))
            && freq.contains_key(&(TileSuit::Pinzu, rank + 1))
            && freq.contains_key(&(TileSuit::Pinzu, rank + 2));
        let has_s = freq.contains_key(&(TileSuit::Souzu, rank))
            && freq.contains_key(&(TileSuit::Souzu, rank + 1))
            && freq.contains_key(&(TileSuit::Souzu, rank + 2));
        if has_m && has_p && has_s {
            matches.push(PatternMatch {
                pattern_id: "sanshoku".into(),
                name_zh: "三色同顺".into(),
                base_fan: 2.0,
                rarity: Rarity::Uncommon,
            });
            break;
        }
    }

    // 一气通贯 (Full Straight) - 1-9 in one suit
    for suit in [TileSuit::Manzu, TileSuit::Pinzu, TileSuit::Souzu] {
        let has_full = (1u8..=9u8).all(|rank| freq.contains_key(&(suit, rank)));
        if has_full {
            matches.push(PatternMatch {
                pattern_id: "ittsu".into(),
                name_zh: "一气通贯".into(),
                base_fan: 2.0,
                rarity: Rarity::Uncommon,
            });
            break;
        }
    }

    // 七对子 (Seven Pairs)
    if tiles.len() == 14 {
        let all_pairs = freq.values().all(|&c| c == 2);
        if all_pairs && freq.len() == 7 {
            matches.push(PatternMatch {
                pattern_id: "chiitoitsu".into(),
                name_zh: "七对子".into(),
                base_fan: 2.0,
                rarity: Rarity::Uncommon,
            });
        }
    }
}

fn check_suit_patterns(
    freq: &HashMap<(TileSuit, u8), usize>,
    tiles: &[Tile],
    matches: &mut Vec<PatternMatch>,
) {
    let mut suit_counts: HashMap<TileSuit, usize> = HashMap::new();
    let mut has_honors = false;

    for tile in tiles {
        *suit_counts.entry(tile.suit).or_insert(0) += 1;
        if matches!(tile.suit, TileSuit::Wind | TileSuit::Dragon) {
            has_honors = true;
        }
    }

    let manzu_count = suit_counts.get(&TileSuit::Manzu).copied().unwrap_or(0);
    let pinzu_count = suit_counts.get(&TileSuit::Pinzu).copied().unwrap_or(0);
    let souzu_count = suit_counts.get(&TileSuit::Souzu).copied().unwrap_or(0);

    // 混一色 (Half Flush) - tiles from one numbered suit + honors
    if has_honors {
        if manzu_count > 0 && pinzu_count == 0 && souzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "honitsu".into(),
                name_zh: "混一色".into(),
                base_fan: 3.0,
                rarity: Rarity::Uncommon,
            });
        } else if pinzu_count > 0 && manzu_count == 0 && souzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "honitsu".into(),
                name_zh: "混一色".into(),
                base_fan: 3.0,
                rarity: Rarity::Uncommon,
            });
        } else if souzu_count > 0 && manzu_count == 0 && pinzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "honitsu".into(),
                name_zh: "混一色".into(),
                base_fan: 3.0,
                rarity: Rarity::Uncommon,
            });
        }
    }

    // 清一色 (Full Flush) - all tiles from one numbered suit, no honors
    if !has_honors {
        if manzu_count > 0 && pinzu_count == 0 && souzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "chinitsu".into(),
                name_zh: "清一色".into(),
                base_fan: 6.0,
                rarity: Rarity::Rare,
            });
        } else if pinzu_count > 0 && manzu_count == 0 && souzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "chinitsu".into(),
                name_zh: "清一色".into(),
                base_fan: 6.0,
                rarity: Rarity::Rare,
            });
        } else if souzu_count > 0 && manzu_count == 0 && pinzu_count == 0 {
            matches.push(PatternMatch {
                pattern_id: "chinitsu".into(),
                name_zh: "清一色".into(),
                base_fan: 6.0,
                rarity: Rarity::Rare,
            });
        }
    }

    // 绿一色 (All Green) - only 2,3,4,6,8 of Souzu + Green Dragon
    let all_green = tiles.iter().all(|t| {
        matches!(t.suit, TileSuit::Souzu) && [2, 3, 4, 6, 8].contains(&t.rank)
            || t.suit == TileSuit::Dragon && t.rank == 2 // Green dragon
    });
    if all_green && !tiles.is_empty() {
        matches.push(PatternMatch {
            pattern_id: "ryuuiisou".into(),
            name_zh: "绿一色".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }
}

fn check_honor_patterns(
    freq: &HashMap<(TileSuit, u8), usize>,
    tiles: &[Tile],
    matches: &mut Vec<PatternMatch>,
) {
    // Count dragon and wind triplets/quads
    let mut dragon_pons = 0;
    let mut wind_pons = 0;

    for (&(suit, rank), &count) in freq {
        if count >= 3 {
            match suit {
                TileSuit::Dragon => dragon_pons += 1,
                TileSuit::Wind => wind_pons += 1,
                _ => {}
            }
        }
    }

    // 役牌: Dragon Pon (each dragon triplet is a yaku)
    for rank in 1u8..=3u8 {
        let count = freq.get(&(TileSuit::Dragon, rank)).copied().unwrap_or(0);
        if count >= 3 {
            let names = ["白板", "發", "中"];
            matches.push(PatternMatch {
                pattern_id: format!("yakuhai_dragon_{}", rank),
                name_zh: names[(rank - 1) as usize].into(),
                base_fan: 1.0,
                rarity: Rarity::Common,
            });
        }
    }

    // 役牌: Wind Pon (round wind or seat wind)
    for rank in 1u8..=4u8 {
        let count = freq.get(&(TileSuit::Wind, rank)).copied().unwrap_or(0);
        if count >= 3 {
            let names = ["东", "南", "西", "北"];
            matches.push(PatternMatch {
                pattern_id: format!("yakuhai_wind_{}", rank),
                name_zh: format!("{}风刻", names[(rank - 1) as usize]),
                base_fan: 1.0,
                rarity: Rarity::Common,
            });
        }
    }

    // 小三元 (Little Three Dragons) - 2 dragon pons + 1 dragon pair
    if dragon_pons >= 2 {
        let dragon_pairs = (1u8..=3u8)
            .filter(|&rank| freq.get(&(TileSuit::Dragon, rank)).copied().unwrap_or(0) == 2)
            .count();
        if dragon_pairs >= 1 {
            matches.push(PatternMatch {
                pattern_id: "shousangen".into(),
                name_zh: "小三元".into(),
                base_fan: 4.0,
                rarity: Rarity::Rare,
            });
        }
    }

    // 大三元 (Big Three Dragons) - all 3 dragon triplets
    if dragon_pons >= 3 {
        matches.push(PatternMatch {
            pattern_id: "daisangen".into(),
            name_zh: "大三元".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 小四喜 (Little Four Winds) - 3 wind pons + 1 wind pair
    if wind_pons >= 3 {
        let wind_pairs = (1u8..=4u8)
            .filter(|&rank| freq.get(&(TileSuit::Wind, rank)).copied().unwrap_or(0) == 2)
            .count();
        if wind_pairs >= 1 {
            matches.push(PatternMatch {
                pattern_id: "shousuushii".into(),
                name_zh: "小四喜".into(),
                base_fan: 13.0,
                rarity: Rarity::Legendary,
            });
        }
    }

    // 大四喜 (Big Four Winds) - all 4 wind triplets
    if wind_pons >= 4 {
        matches.push(PatternMatch {
            pattern_id: "daisuushii".into(),
            name_zh: "大四喜".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 字一色 (All Honors) - only wind and dragon tiles
    if !tiles.is_empty() && tiles.iter().all(|t| matches!(t.suit, TileSuit::Wind | TileSuit::Dragon)) {
        matches.push(PatternMatch {
            pattern_id: "tsuuiisou".into(),
            name_zh: "字一色".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 清老头 (All Terminals) - only 1 and 9 tiles
    let all_terminals = !tiles.is_empty() && tiles.iter().all(|t| {
        matches!(t.suit, TileSuit::Manzu | TileSuit::Pinzu | TileSuit::Souzu) && (t.rank == 1 || t.rank == 9)
    });
    if all_terminals {
        matches.push(PatternMatch {
            pattern_id: "chinroutou".into(),
            name_zh: "清老头".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 混老头 (Mixed Terminals) - terminals and honors only, with honors
    let mixed_terminals = !tiles.is_empty() && tiles.iter().all(|t| t.is_terminal_or_honor()) && has_honor(tiles);
    if mixed_terminals {
        matches.push(PatternMatch {
            pattern_id: "honroutou".into(),
            name_zh: "混老头".into(),
            base_fan: 2.0,
            rarity: Rarity::Uncommon,
        });
    }
}

fn check_special_patterns(
    tiles: &[Tile],
    all_played_tiles: &[Tile],
    matches: &mut Vec<PatternMatch>,
) {
    // 国士无双 (Thirteen Orphans)
    if tiles.len() == 14 {
        let required: Vec<(TileSuit, u8)> = vec![
            (TileSuit::Manzu, 1), (TileSuit::Manzu, 9),
            (TileSuit::Pinzu, 1), (TileSuit::Pinzu, 9),
            (TileSuit::Souzu, 1), (TileSuit::Souzu, 9),
            (TileSuit::Wind, 1), (TileSuit::Wind, 2),
            (TileSuit::Wind, 3), (TileSuit::Wind, 4),
            (TileSuit::Dragon, 1), (TileSuit::Dragon, 2), (TileSuit::Dragon, 3),
        ];
        let tile_types: std::collections::HashSet<(TileSuit, u8)> =
            tiles.iter().map(|t| (t.suit, t.rank)).collect();
        let has_all = required.iter().all(|r| tile_types.contains(r));
        let has_pair = tile_types.len() == 13 && tiles.len() == 14;
        if has_all && has_pair {
            matches.push(PatternMatch {
                pattern_id: "kokushi".into(),
                name_zh: "国士无双".into(),
                base_fan: 13.0,
                rarity: Rarity::Legendary,
            });
        }
    }

    // 四暗刻 (Four Concealed Triplets) - 4 triplets in hand
    let freq = build_freq(tiles);
    let pon_count = freq.values().filter(|&&c| c >= 3).count();
    if pon_count >= 4 {
        matches.push(PatternMatch {
            pattern_id: "suuankou".into(),
            name_zh: "四暗刻".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 三暗刻 (Three Concealed Triplets)
    if pon_count >= 3 && pon_count < 4 {
        matches.push(PatternMatch {
            pattern_id: "sanankou".into(),
            name_zh: "三暗刻".into(),
            base_fan: 2.0,
            rarity: Rarity::Uncommon,
        });
    }

    // 四杠子 (Four Quads)
    let kan_count = freq.values().filter(|&&c| c >= 4).count();
    if kan_count >= 4 {
        matches.push(PatternMatch {
            pattern_id: "suukantsu".into(),
            name_zh: "四杠子".into(),
            base_fan: 13.0,
            rarity: Rarity::Legendary,
        });
    }

    // 三杠子 (Three Quads)
    if kan_count >= 3 && kan_count < 4 {
        matches.push(PatternMatch {
            pattern_id: "sankantsu".into(),
            name_zh: "三杠子".into(),
            base_fan: 2.0,
            rarity: Rarity::Uncommon,
        });
    }

    // 九莲宝灯 (Nine Gates) - 1112345678999 + any one tile of the same suit
    if tiles.len() >= 14 {
        for suit in [TileSuit::Manzu, TileSuit::Pinzu, TileSuit::Souzu] {
            if check_nine_gates(tiles, suit) {
                matches.push(PatternMatch {
                    pattern_id: "chuuren".into(),
                    name_zh: "九莲宝灯".into(),
                    base_fan: 13.0,
                    rarity: Rarity::Legendary,
                });
                break;
            }
        }
    }

    // 二杯口 (Two Sets of Identical Sequences)
    // 一杯口 (One Set of Identical Sequences)
    let mut identical_chi_count = 0;
    let mut counted: std::collections::HashSet<(TileSuit, u8, u8, u8)> = std::collections::HashSet::new();
    for (&(suit1, rank1), &count1) in &freq {
        if !is_numbered_suit(suit1) || rank1 > 7 || count1 < 2 {
            continue;
        }
        let has_next1 = freq.get(&(suit1, rank1 + 1)).copied().unwrap_or(0) >= 2;
        let has_next2 = freq.get(&(suit1, rank1 + 2)).copied().unwrap_or(0) >= 2;
        if has_next1 && has_next2 {
            let key = (suit1, rank1, rank1 + 1, rank1 + 2);
            if !counted.contains(&key) {
                counted.insert(key);
                identical_chi_count += 1;
            }
        }
    }

    if identical_chi_count >= 2 {
        matches.push(PatternMatch {
            pattern_id: "ryanpeikou".into(),
            name_zh: "二杯口".into(),
            base_fan: 3.0,
            rarity: Rarity::Rare,
        });
    } else if identical_chi_count == 1 {
        matches.push(PatternMatch {
            pattern_id: "iipeikou".into(),
            name_zh: "一杯口".into(),
            base_fan: 1.0,
            rarity: Rarity::Common,
        });
    }
}

fn check_nine_gates(tiles: &[Tile], suit: TileSuit) -> bool {
    let suit_tiles: Vec<&Tile> = tiles.iter().filter(|t| t.suit == suit).collect();
    if suit_tiles.len() != tiles.len() {
        return false;
    }

    // Need: 1112345678999 + one extra tile of same suit
    let mut freq = HashMap::new();
    for t in &suit_tiles {
        *freq.entry(t.rank).or_insert(0usize) += 1;
    }

    // Base pattern: 1×3, 2×1, 3×1, 4×1, 5×1, 6×1, 7×1, 8×1, 9×3 = 14 tiles
    let base = [(1u8, 3), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 3)];

    // Check if removing one tile from freq results in the base pattern
    for rank in 1u8..=9u8 {
        let count = freq.get(&rank).copied().unwrap_or(0);
        if count == 0 {
            continue;
        }
        let mut test_freq = freq.clone();
        *test_freq.entry(rank).or_insert(0) -= 1;
        if test_freq.get(&rank) == Some(&0) {
            test_freq.remove(&rank);
        }

        let matches_base = base.iter().all(|&(r, c)| test_freq.get(&r).copied().unwrap_or(0) == c);
        if matches_base {
            return true;
        }
    }
    false
}

fn is_numbered_suit(suit: TileSuit) -> bool {
    matches!(suit, TileSuit::Manzu | TileSuit::Pinzu | TileSuit::Souzu)
}

fn has_honor(tiles: &[Tile]) -> bool {
    tiles.iter().any(|t| matches!(t.suit, TileSuit::Wind | TileSuit::Dragon))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tile::Tile;

    #[test]
    fn test_tanyao() {
        let tiles = vec![
            Tile::new(TileSuit::Manzu, 2, 0),
            Tile::new(TileSuit::Manzu, 3, 1),
            Tile::new(TileSuit::Manzu, 4, 2),
            Tile::new(TileSuit::Pinzu, 5, 3),
            Tile::new(TileSuit::Pinzu, 5, 4),
        ];
        let matches = check_patterns(&tiles, &tiles, &tiles);
        assert!(matches.iter().any(|m| m.pattern_id == "tanyao"));
    }

    #[test]
    fn test_dragon_pon() {
        let tiles = vec![
            Tile::new(TileSuit::Dragon, 3, 0), // 中
            Tile::new(TileSuit::Dragon, 3, 1),
            Tile::new(TileSuit::Dragon, 3, 2),
            Tile::new(TileSuit::Pinzu, 5, 3),
            Tile::new(TileSuit::Pinzu, 5, 4),
        ];
        let matches = check_patterns(&tiles, &tiles, &tiles);
        assert!(matches.iter().any(|m| m.pattern_id == "yakuhai_dragon_3"));
    }

    #[test]
    fn test_chinitsu() {
        let tiles = vec![
            Tile::new(TileSuit::Manzu, 1, 0),
            Tile::new(TileSuit::Manzu, 2, 1),
            Tile::new(TileSuit::Manzu, 3, 2),
            Tile::new(TileSuit::Manzu, 4, 3),
            Tile::new(TileSuit::Manzu, 5, 4),
            Tile::new(TileSuit::Manzu, 6, 5),
            Tile::new(TileSuit::Manzu, 7, 6),
            Tile::new(TileSuit::Manzu, 8, 7),
            Tile::new(TileSuit::Manzu, 9, 8),
            Tile::new(TileSuit::Manzu, 9, 9),
            Tile::new(TileSuit::Manzu, 9, 10),
            Tile::new(TileSuit::Manzu, 7, 11),
            Tile::new(TileSuit::Manzu, 8, 12),
            Tile::new(TileSuit::Manzu, 9, 13),
        ];
        let matches = check_patterns(&tiles, &tiles, &tiles);
        assert!(matches.iter().any(|m| m.pattern_id == "chinitsu"));
    }
}

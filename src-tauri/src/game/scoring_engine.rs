use crate::models::artifact::{Artifact, ArtifactEffect};
use crate::models::hand::{MeldKind, Play};
use crate::models::tile::Tile;
use crate::models::scoring::ScoreResult;
use crate::game::pattern_checker;

/// Full scoring pipeline: tile fu → pattern fan → artifact effects → boss modifiers
pub fn calculate_score(
    play: &Play,
    artifacts: &[Artifact],
    all_played_tiles: &[Tile],
    hand_tiles: &[Tile],
) -> ScoreResult {
    let tiles: Vec<Tile> = play.all_tiles().into_iter().copied().collect();
    let mut breakdown: Vec<String> = Vec::new();

    // ── Phase 1: Tile Fu ──
    let mut tile_fu: u64 = 0;
    for tile in &tiles {
        let base = tile.base_fu();
        let buff = tile.buff_fu.max(0) as u64;
        tile_fu += base + buff;
    }
    if tile_fu > 0 {
        breakdown.push(format!("牌面符: +{}", tile_fu));
    }

    // ── Phase 2: Meld structure fan ──
    let mut base_fan: f64 = 2.0;
    for meld in &play.melds {
        let bonus = match meld.kind {
            MeldKind::Pon => 2.0,
            MeldKind::Kan => 4.0,
            MeldKind::Chi => 1.0,
            MeldKind::Pair => 0.0,
        };
        base_fan += bonus;
    }
    breakdown.push(format!("结构番: {:.1}", base_fan));

    // ── Phase 3: Pattern (yaku) fan ──
    let pattern_matches = pattern_checker::check_patterns(&tiles, all_played_tiles, hand_tiles);
    let mut pattern_fan: f64 = 0.0;
    for pm in &pattern_matches {
        pattern_fan += pm.base_fan;
        breakdown.push(format!("{}: +{:.1}番", pm.name_zh, pm.base_fan));
    }

    // ── Phase 4: Artifact effects (order matters!) ──
    let mut artifact_fu: u64 = 0;
    let mut artifact_fan: f64 = 0.0;
    let mut cumulative_mult: f64 = 1.0;

    for artifact in artifacts {
        match &artifact.effect {
            ArtifactEffect::AddFu(v) => {
                artifact_fu += v;
                breakdown.push(format!("{}: +{}符", artifact.name_zh, v));
            }
            ArtifactEffect::AddFan(v) => {
                artifact_fan += v;
                breakdown.push(format!("{}: +{:.1}番", artifact.name_zh, v));
            }
            ArtifactEffect::MultiplyMult(v) => {
                cumulative_mult *= v;
                breakdown.push(format!("{}: ×{:.1}", artifact.name_zh, v));
            }
            ArtifactEffect::ConditionalAddFan { condition, value } => {
                if evaluate_condition(condition, play, &tiles, hand_tiles) {
                    artifact_fan += value;
                    breakdown.push(format!("{}: +{:.1}番 ✓", artifact.name_zh, value));
                }
            }
            ArtifactEffect::ScalingAddFu { per_round, current } => {
                let bonus = *current;
                artifact_fu += bonus;
                if bonus > 0 {
                    breakdown.push(format!("{}: +{}符 (累计)", artifact.name_zh, bonus));
                }
            }
            ArtifactEffect::ScalingAddFan { per_round: _, current } => {
                artifact_fan += *current;
                if *current > 0.0 {
                    breakdown.push(format!("{}: +{:.1}番 (累计)", artifact.name_zh, current));
                }
            }
            ArtifactEffect::PatternBonus { pattern_id, bonus } => {
                if pattern_matches.iter().any(|pm| pm.pattern_id == *pattern_id) {
                    artifact_fan += bonus;
                    breakdown.push(format!("{}: +{:.1}番 (牌型加成)", artifact.name_zh, bonus));
                }
            }
        }
    }

    // ── Final calculation ──
    let total_fu = tile_fu + artifact_fu;
    let total_fan = base_fan + pattern_fan + artifact_fan;

    // Meld-type multiplier based on play composition
    let meld_multiplier = compute_meld_multiplier(play);
    if meld_multiplier > 1.0 {
        breakdown.push(format!("牌型倍率: ×{:.0}", meld_multiplier));
    }

    let final_score = (total_fu as f64 * total_fan * cumulative_mult * meld_multiplier) as u64;

    ScoreResult {
        fu: total_fu,
        fan: total_fan,
        mult: cumulative_mult * meld_multiplier,
        final_score,
        breakdown,
    }
}

fn evaluate_condition(condition: &str, play: &Play, tiles: &[Tile], _hand_tiles: &[Tile]) -> bool {
    match condition {
        "has_dragon" => tiles.iter().any(|t| t.suit == crate::models::tile::TileSuit::Dragon),
        "has_wind" => tiles.iter().any(|t| t.suit == crate::models::tile::TileSuit::Wind),
        "has_pon" => play.melds.iter().any(|m| m.kind == MeldKind::Pon),
        "has_kan" => play.melds.iter().any(|m| m.kind == MeldKind::Kan),
        "has_chi" => play.melds.iter().any(|m| m.kind == MeldKind::Chi),
        "has_terminal" => tiles.iter().any(|t| matches!(t.suit, crate::models::tile::TileSuit::Manzu | crate::models::tile::TileSuit::Pinzu | crate::models::tile::TileSuit::Souzu) && (t.rank == 1 || t.rank == 9)),
        "all_same_suit" => {
            let suits: std::collections::HashSet<_> = tiles.iter().map(|t| t.suit).collect();
            suits.len() == 1
        }
        "has_manzu" => tiles.iter().any(|t| t.suit == crate::models::tile::TileSuit::Manzu),
        "has_pinzu" => tiles.iter().any(|t| t.suit == crate::models::tile::TileSuit::Pinzu),
        "has_souzu" => tiles.iter().any(|t| t.suit == crate::models::tile::TileSuit::Souzu),
        _ => false,
    }
}

fn compute_meld_multiplier(play: &Play) -> f64 {
    let melds = &play.melds;
    let has_pair = play.pair.is_some();

    // ── Single-meld + Pair: Chi×2, Pon×4, Kan×8 ──
    if has_pair && melds.len() == 1 {
        return match melds[0].kind {
            MeldKind::Kan => 8.0,
            MeldKind::Pon => 4.0,
            MeldKind::Chi => 2.0,
            MeldKind::Pair => 1.0,
        };
    }

    // ── Three Pairs (AA+BB+CC) ──
    if !has_pair && melds.len() == 3 && melds.iter().all(|m| m.kind == MeldKind::Pair) {
        return compute_three_pairs_multiplier(melds);
    }

    // ── Two Melds, no pair: Chi+Chi, Pon+Pon, or Chi+Pon ──
    if !has_pair && melds.len() == 2 {
        let kinds: Vec<_> = melds.iter().map(|m| m.kind).collect();
        if kinds[0] == MeldKind::Chi && kinds[1] == MeldKind::Chi {
            return compute_chi_chi_multiplier(melds);
        }
        if kinds[0] == MeldKind::Pon && kinds[1] == MeldKind::Pon {
            return compute_pon_pon_multiplier(melds);
        }
        // Chi+Pon (either order)
        if kinds.contains(&MeldKind::Chi) && kinds.contains(&MeldKind::Pon) {
            return compute_chi_pon_multiplier(melds);
        }
    }

    1.0
}

fn compute_chi_chi_multiplier(melds: &[crate::models::hand::Meld]) -> f64 {
    let tiles_a = &melds[0].tiles;
    let tiles_b = &melds[1].tiles;
    let suit_a = tiles_a[0].suit;
    let suit_b = tiles_b[0].suit;

    // Sort ranks for each chi
    let mut ranks_a: Vec<u8> = tiles_a.iter().map(|t| t.rank).collect();
    let mut ranks_b: Vec<u8> = tiles_b.iter().map(|t| t.rank).collect();
    ranks_a.sort();
    ranks_b.sort();

    let same_suit = suit_a == suit_b;
    let same_ranks = ranks_a == ranks_b;

    if same_suit && same_ranks {
        8.0 // Same suit + same ranks
    } else if same_suit {
        6.0 // Same suit, different ranks
    } else {
        4.0 // Different suits
    }
}

fn compute_pon_pon_multiplier(melds: &[crate::models::hand::Meld]) -> f64 {
    let tiles_a = &melds[0].tiles;
    let tiles_b = &melds[1].tiles;
    let suit_a = tiles_a[0].suit;
    let suit_b = tiles_b[0].suit;
    let rank_a = tiles_a[0].rank;
    let rank_b = tiles_b[0].rank;

    let same_suit = suit_a == suit_b;

    if same_suit {
        let diff = (rank_a as i16 - rank_b as i16).unsigned_abs() as u8;
        if diff == 1 {
            8.0 // Same suit + adjacent ranks
        } else {
            6.0 // Same suit, non-adjacent
        }
    } else {
        4.0 // Different suits
    }
}

fn compute_chi_pon_multiplier(melds: &[crate::models::hand::Meld]) -> f64 {
    let (chi_meld, pon_meld) = if melds[0].kind == MeldKind::Chi {
        (&melds[0], &melds[1])
    } else {
        (&melds[1], &melds[0])
    };

    let chi_suit = chi_meld.tiles[0].suit;
    let pon_suit = pon_meld.tiles[0].suit;
    let pon_rank = pon_meld.tiles[0].rank;
    let same_suit = chi_suit == pon_suit;

    if same_suit {
        // Check if pon rank matches any chi tile rank
        let chi_ranks: Vec<u8> = chi_meld.tiles.iter().map(|t| t.rank).collect();
        if chi_ranks.contains(&pon_rank) {
            8.0 // Same suit + rank match
        } else {
            6.0 // Same suit, no rank match
        }
    } else {
        4.0 // Different suits
    }
}

fn compute_three_pairs_multiplier(melds: &[crate::models::hand::Meld]) -> f64 {
    let suits: Vec<_> = melds.iter().map(|m| m.tiles[0].suit).collect();
    let ranks: Vec<u8> = melds.iter().map(|m| m.tiles[0].rank).collect();

    let same_suit_count = {
        let mut count = 0u8;
        for i in 0..3 {
            for j in i + 1..3 {
                if suits[i] == suits[j] {
                    count += 1;
                }
            }
        }
        count
    };

    // All three different suits
    if same_suit_count == 0 {
        return 4.0;
    }

    // All three same suit
    if suits[0] == suits[1] && suits[1] == suits[2] {
        let mut sorted_ranks = ranks.clone();
        sorted_ranks.sort();
        let adjacent = sorted_ranks[0] + 1 == sorted_ranks[1]
            && sorted_ranks[1] + 1 == sorted_ranks[2];
        if adjacent {
            8.0 // Three same suit + adjacent ranks
        } else {
            7.0 // Three same suit
        }
    } else {
        // Exactly one pair of same suit
        // Find which two share a suit
        let mut pair_idx = (0, 1);
        for i in 0..3 {
            for j in i + 1..3 {
                if suits[i] == suits[j] {
                    pair_idx = (i, j);
                }
            }
        }
        let adjacent = (ranks[pair_idx.0] as i16 - ranks[pair_idx.1] as i16).unsigned_abs() == 1;
        if adjacent {
            6.0 // Two same suit + adjacent
        } else {
            5.0 // Two same suit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tile::{Tile, TileSuit};
    use crate::models::hand::{Meld, MeldKind, Play};

    fn make_chi_play() -> Play {
        Play::new(
            vec![Meld::new(
                MeldKind::Chi,
                vec![
                    Tile::new(TileSuit::Manzu, 1, 0),
                    Tile::new(TileSuit::Manzu, 2, 1),
                    Tile::new(TileSuit::Manzu, 3, 2),
                ],
            )],
            Some(Meld::new(
                MeldKind::Pair,
                vec![Tile::new(TileSuit::Pinzu, 5, 3), Tile::new(TileSuit::Pinzu, 5, 4)],
            )),
        )
    }

    #[test]
    fn test_basic_scoring_no_artifacts() {
        let play = make_chi_play();
        let result = calculate_score(&play, &[], &[], &[]);
        assert!(result.final_score > 0);
        assert!(result.fu > 0);
        assert!(result.fan > 0.0);
        assert!(result.mult > 1.0); // includes meld-type multiplier
    }

    #[test]
    fn test_scoring_with_add_fu_artifact() {
        let play = make_chi_play();
        let base = calculate_score(&play, &[], &[], &[]);
        let with_artifact = calculate_score(
            &play,
            &[Artifact {
                id: "test".into(),
                name_zh: "测试遗物".into(),
                name_en: "Test".into(),
                description_zh: "test".into(),
                rarity: crate::models::artifact::Rarity::Common,
                cost: 50,
                sell_value: 25,
                effect: ArtifactEffect::AddFu(20),
            }],
            &[],
            &[],
        );
        assert!(with_artifact.final_score > base.final_score);
    }

    #[test]
    fn test_scoring_with_multiply_artifact() {
        let play = make_chi_play();
        let base = calculate_score(&play, &[], &[], &[]);
        let with_mult = calculate_score(
            &play,
            &[Artifact {
                id: "test".into(),
                name_zh: "测试遗物".into(),
                name_en: "Test".into(),
                description_zh: "test".into(),
                rarity: crate::models::artifact::Rarity::Common,
                cost: 50,
                sell_value: 25,
                effect: ArtifactEffect::MultiplyMult(2.0),
            }],
            &[],
            &[],
        );
        assert!(with_mult.final_score >= base.final_score * 2);
    }
}

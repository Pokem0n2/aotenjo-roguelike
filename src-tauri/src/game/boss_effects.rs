use crate::models::boss::{Boss, BossGimmick};
use crate::models::tile::TileSuit;

/// 8 bosses, one per round. Assigned in sequence East 1-4, South 1-4, then repeats for West/North.
pub fn get_boss_for_round(wind_index: u32, round: u8) -> Boss {
    // 8 unique bosses cycle every 8 rounds
    let idx = ((wind_index * 4) + (round - 1) as u32) % 8;
    all_bosses()[idx as usize].clone()
}

pub fn all_bosses() -> Vec<Boss> {
    vec![
        Boss {
            id: "the_blind".into(),
            name_zh: "盲目之风".into(),
            name_en: "The Blind Wind".into(),
            gimmick: BossGimmick::BlindSuit(TileSuit::Souzu),
            score_multiplier: 1.0,
        },
        Boss {
            id: "the_hungry".into(),
            name_zh: "饥饿之鬼".into(),
            name_en: "The Hungry Ghost".into(),
            gimmick: BossGimmick::ReducedPlays(3),
            score_multiplier: 1.0,
        },
        Boss {
            id: "the_miser".into(),
            name_zh: "吝啬之徒".into(),
            name_en: "The Miser".into(),
            gimmick: BossGimmick::TaxPerPlay(30),
            score_multiplier: 1.0,
        },
        Boss {
            id: "the_wall".into(),
            name_zh: "铁壁之将".into(),
            name_en: "The Iron General".into(),
            gimmick: BossGimmick::ScoreThresholdScale(1.5),
            score_multiplier: 1.5,
        },
        Boss {
            id: "the_thief".into(),
            name_zh: "无影之贼".into(),
            name_en: "The Shadow Thief".into(),
            gimmick: BossGimmick::NoDiscard,
            score_multiplier: 1.0,
        },
        Boss {
            id: "the_fusion".into(),
            name_zh: "融合之龙".into(),
            name_en: "The Fusion Dragon".into(),
            gimmick: BossGimmick::ForcedPattern("honitsu".into()),
            score_multiplier: 1.2,
        },
        Boss {
            id: "the_storm".into(),
            name_zh: "暴风之神".into(),
            name_en: "The Storm God".into(),
            gimmick: BossGimmick::BlindSuit(TileSuit::Pinzu),
            score_multiplier: 1.0,
        },
        Boss {
            id: "the_emperor".into(),
            name_zh: "天帝".into(),
            name_en: "The Emperor".into(),
            gimmick: BossGimmick::ReducedPlays(3),
            score_multiplier: 1.3,
        },
    ]
}

/// Apply boss gimmick to game parameters. Returns (adjusted_max_plays, adjusted_target, tax_per_play).
pub fn apply_boss_effects(
    boss: &Boss,
    base_max_plays: u8,
    base_target: u64,
) -> (u8, u64, u32) {
    let mut max_plays = base_max_plays;
    let mut target = base_target;
    let mut tax = 0u32;

    match &boss.gimmick {
        BossGimmick::ReducedPlays(n) => {
            max_plays = max_plays.min(*n);
        }
        BossGimmick::ScoreThresholdScale(scale) => {
            target = ((target as f64) * scale) as u64;
        }
        BossGimmick::TaxPerPlay(t) => {
            tax = *t;
        }
        BossGimmick::BlindSuit(_) => {} // Handled in frontend: hide suit display
        BossGimmick::NoDiscard => {}     // Handled in skip_play: disabled
        BossGimmick::ForcedPattern(_) => {} // Handled in scoring: bonus fan if matched
    }

    (max_plays, target, tax)
}

/// Get bonus fan when a forced-pattern boss is satisfied
pub fn boss_pattern_bonus(boss: &Boss, matched_patterns: &[String]) -> f64 {
    if let BossGimmick::ForcedPattern(pattern_id) = &boss.gimmick {
        if matched_patterns.iter().any(|p| p == pattern_id) {
            return 3.0;
        }
    }
    0.0
}

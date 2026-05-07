use crate::models::artifact::{Artifact, Rarity};
use crate::models::gadget::Gadget;
use crate::game::artifact_effects::all_artifacts;
use crate::game::gadget_effects::{all_gadgets, gadget_cost};

use rand::prelude::IndexedRandom;
use rand::Rng;

/// A single item in the shop
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShopItem {
    pub item_type: ShopItemType,
    pub cost: u32,
    pub sold: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ShopItemType {
    Artifact(Artifact),
    Gadget(Gadget),
}

/// Generate a fresh shop inventory for a given round
pub fn generate_shop(round_progress: u32, rng: &mut impl Rng) -> Vec<ShopItem> {
    let mut items = Vec::new();

    // 3 artifacts, 2 gadgets
    let artifacts = roll_artifacts(3, round_progress, rng);
    for artifact in artifacts {
        items.push(ShopItem {
            cost: artifact.cost,
            item_type: ShopItemType::Artifact(artifact),
            sold: false,
        });
    }

    let gadgets = roll_gadgets(2, rng);
    for gadget in gadgets {
        items.push(ShopItem {
            cost: gadget_cost(&gadget),
            item_type: ShopItemType::Gadget(gadget),
            sold: false,
        });
    }

    items
}

fn roll_artifacts(count: usize, round_progress: u32, rng: &mut impl Rng) -> Vec<Artifact> {
    let all = all_artifacts();
    let mut pool: Vec<&Artifact> = Vec::new();

    // Weight by rarity: higher rounds → better rarity odds
    let (common_w, uncommon_w, rare_w, legend_w) = if round_progress < 4 {
        (60, 30, 8, 2)
    } else if round_progress < 8 {
        (40, 35, 18, 7)
    } else if round_progress < 12 {
        (25, 30, 30, 15)
    } else {
        (15, 25, 35, 25)
    };

    for artifact in &all {
        let weight = match artifact.rarity {
            Rarity::Common => common_w,
            Rarity::Uncommon => uncommon_w,
            Rarity::Rare => rare_w,
            Rarity::Legendary => legend_w,
        };
        for _ in 0..weight {
            pool.push(artifact);
        }
    }

    let mut chosen = Vec::new();
    let mut attempts = 0;
    while chosen.len() < count && attempts < 50 {
        if let Some(&artifact) = pool.choose(rng) {
            if !chosen.iter().any(|a: &Artifact| a.id == artifact.id) {
                chosen.push(artifact.clone());
            }
        }
        attempts += 1;
    }
    chosen
}

fn roll_gadgets(count: usize, rng: &mut impl Rng) -> Vec<Gadget> {
    let all = all_gadgets();
    let mut chosen = Vec::new();
    let mut attempts = 0;
    while chosen.len() < count && attempts < 30 {
        if let Some(gadget) = all.choose(rng) {
            if !chosen.iter().any(|g: &Gadget| g.id == gadget.id) {
                chosen.push(gadget.clone());
            }
        }
        attempts += 1;
    }
    chosen
}

use crate::models::gadget::{Gadget, GadgetEffect};
use crate::models::artifact::Rarity;

/// 10 gadgets — consumable items that manipulate tiles/wall
pub fn all_gadgets() -> Vec<Gadget> {
    vec![
        Gadget {
            id: "hammer".into(),
            name_zh: "锤子".into(),
            name_en: "Hammer".into(),
            description_zh: "销毁手牌中最多3张选定的牌".into(),
            effect: GadgetEffect::DestroyTiles { count: 3 },
        },
        Gadget {
            id: "swapper".into(),
            name_zh: "交换器".into(),
            name_en: "Swapper".into(),
            description_zh: "将手牌中2张牌交换为牌墙中的牌".into(),
            effect: GadgetEffect::SwapTiles,
        },
        Gadget {
            id: "peek_lens".into(),
            name_zh: "透镜".into(),
            name_en: "Peek Lens".into(),
            description_zh: "查看牌墙顶部5张牌".into(),
            effect: GadgetEffect::PeekWall { count: 5 },
        },
        Gadget {
            id: "transformer".into(),
            name_zh: "变形符".into(),
            name_en: "Transformer".into(),
            description_zh: "将手牌中1张牌变为同花色的另一张牌".into(),
            effect: GadgetEffect::TransformTile,
        },
        Gadget {
            id: "fu_charm".into(),
            name_zh: "符力护符".into(),
            name_en: "Fu Charm".into(),
            description_zh: "本局所有牌+5符值".into(),
            effect: GadgetEffect::BuffTile { fu_bonus: 5, mult_bonus: 0 },
        },
        Gadget {
            id: "mult_charm".into(),
            name_zh: "番力护符".into(),
            name_en: "Mult Charm".into(),
            description_zh: "本局所有牌+2番值".into(),
            effect: GadgetEffect::BuffTile { fu_bonus: 0, mult_bonus: 2 },
        },
        Gadget {
            id: "great_hammer".into(),
            name_zh: "大锤".into(),
            name_en: "Great Hammer".into(),
            description_zh: "销毁手牌中最多5张选定的牌".into(),
            effect: GadgetEffect::DestroyTiles { count: 5 },
        },
        Gadget {
            id: "magnet".into(),
            name_zh: "磁石".into(),
            name_en: "Magnet".into(),
            description_zh: "从牌墙摸5张牌加入手牌".into(),
            effect: GadgetEffect::DrawExtra { count: 5 },
        },
        Gadget {
            id: "purifier".into(),
            name_zh: "净化符".into(),
            name_en: "Purifier".into(),
            description_zh: "移除当前Boss的特殊效果".into(),
            effect: GadgetEffect::DisableBoss,
        },
        Gadget {
            id: "fortune_coin".into(),
            name_zh: "招财符".into(),
            name_en: "Fortune Coin".into(),
            description_zh: "获得100金币".into(),
            effect: GadgetEffect::GainCurrency { amount: 100 },
        },
    ]
}

/// Get gadget cost by effect type (for shop pricing)
pub fn gadget_cost(gadget: &Gadget) -> u32 {
    match &gadget.effect {
        GadgetEffect::DestroyTiles { count } => 30 + (*count as u32) * 15,
        GadgetEffect::SwapTiles => 50,
        GadgetEffect::PeekWall { count } => 20 + (*count as u32) * 5,
        GadgetEffect::TransformTile => 60,
        GadgetEffect::BuffTile { fu_bonus, mult_bonus } => {
            40 + (*fu_bonus as u32) * 5 + (*mult_bonus as u32) * 15
        }
        GadgetEffect::DrawExtra { count } => 30 + (*count as u32) * 10,
        GadgetEffect::DisableBoss => 80,
        GadgetEffect::GainCurrency { amount } => (*amount as u32) / 3,
    }
}

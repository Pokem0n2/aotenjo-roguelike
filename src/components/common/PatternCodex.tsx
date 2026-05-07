import { getTileImagePathFromSuitRank } from "../Tile/Tile";

interface PatternDef {
  id: string;
  name_zh: string;
  base_fan: number;
  desc: string;
  example: { suit: string; rank: number }[];
}

const PATTERNS: PatternDef[] = [
  { id: "tanyao", name_zh: "断么九", base_fan: 1, desc: "全部为中张牌(2-8)，不含么九和字牌",
    example: [{ suit: "Manzu", rank: 2 }, { suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 4 }] },
  { id: "iipeikou", name_zh: "一杯口", base_fan: 1, desc: "两组完全相同的顺子",
    example: [{ suit: "Pinzu", rank: 3 }, { suit: "Pinzu", rank: 4 }, { suit: "Pinzu", rank: 5 }, { suit: "Pinzu", rank: 3 }, { suit: "Pinzu", rank: 4 }, { suit: "Pinzu", rank: 5 }] },
  { id: "yakuhai_haku", name_zh: "役牌·白", base_fan: 1, desc: "白板刻子(3张)",
    example: [{ suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 1 }] },
  { id: "yakuhai_hatsu", name_zh: "役牌·發", base_fan: 1, desc: "發财刻子(3张)",
    example: [{ suit: "Dragon", rank: 2 }, { suit: "Dragon", rank: 2 }, { suit: "Dragon", rank: 2 }] },
  { id: "yakuhai_chun", name_zh: "役牌·中", base_fan: 1, desc: "红中刻子(3张)",
    example: [{ suit: "Dragon", rank: 3 }, { suit: "Dragon", rank: 3 }, { suit: "Dragon", rank: 3 }] },
  { id: "yakuhai_wind", name_zh: "役牌·场风", base_fan: 1, desc: "场风刻子(如东风局有东风刻子)",
    example: [{ suit: "Wind", rank: 1 }, { suit: "Wind", rank: 1 }, { suit: "Wind", rank: 1 }] },
  { id: "chanta", name_zh: "混全带么", base_fan: 2, desc: "每副面子含么九(1/9)或字牌",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 2 }, { suit: "Manzu", rank: 3 }] },
  { id: "sanshoku", name_zh: "三色同顺", base_fan: 2, desc: "万/筒/索三种花色同数顺子",
    example: [{ suit: "Manzu", rank: 4 }, { suit: "Manzu", rank: 5 }, { suit: "Manzu", rank: 6 }, { suit: "Pinzu", rank: 4 }, { suit: "Souzu", rank: 4 }] },
  { id: "ittsu", name_zh: "一气通贯", base_fan: 2, desc: "同花色1-9连成顺子",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 2 }, { suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 4 }, { suit: "Manzu", rank: 7 }] },
  { id: "toitoi", name_zh: "对对和", base_fan: 2, desc: "全部刻子(无顺子)",
    example: [{ suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 3 }] },
  { id: "sanankou", name_zh: "三暗刻", base_fan: 2, desc: "三个暗刻(非副露的刻子)",
    example: [{ suit: "Pinzu", rank: 5 }, { suit: "Pinzu", rank: 5 }, { suit: "Pinzu", rank: 5 }] },
  { id: "chiitoitsu", name_zh: "七对子", base_fan: 2, desc: "七个不同的对子",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }, { suit: "Pinzu", rank: 7 }, { suit: "Pinzu", rank: 7 }] },
  { id: "honitsu", name_zh: "混一色", base_fan: 3, desc: "只有一种数牌 + 字牌",
    example: [{ suit: "Manzu", rank: 2 }, { suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 4 }, { suit: "Wind", rank: 3 }] },
  { id: "junchan", name_zh: "纯全带么", base_fan: 3, desc: "每副面子含么九(1/9)，无字牌",
    example: [{ suit: "Pinzu", rank: 7 }, { suit: "Pinzu", rank: 8 }, { suit: "Pinzu", rank: 9 }] },
  { id: "ryanpeikou", name_zh: "二杯口", base_fan: 3, desc: "两对完全相同的顺子",
    example: [{ suit: "Souzu", rank: 2 }, { suit: "Souzu", rank: 3 }, { suit: "Souzu", rank: 4 }] },
  { id: "sankantsu", name_zh: "三杠子", base_fan: 3, desc: "三个杠(4张相同的牌×3)",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }] },
  { id: "honroutou", name_zh: "混老头", base_fan: 3, desc: "全部由么九牌和字牌组成",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 9 }, { suit: "Dragon", rank: 3 }] },
  { id: "shousangen", name_zh: "小三元", base_fan: 4, desc: "两种三元牌刻子 + 一对三元牌",
    example: [{ suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 2 }, { suit: "Dragon", rank: 3 }] },
  { id: "chinitsu", name_zh: "清一色", base_fan: 6, desc: "纯一种数牌(无字牌)",
    example: [{ suit: "Souzu", rank: 1 }, { suit: "Souzu", rank: 2 }, { suit: "Souzu", rank: 3 }, { suit: "Souzu", rank: 5 }, { suit: "Souzu", rank: 9 }] },
  { id: "ryuuiisou", name_zh: "绿一色", base_fan: 6, desc: "全绿牌: 2/3/4/6/8索 + 發",
    example: [{ suit: "Souzu", rank: 2 }, { suit: "Souzu", rank: 3 }, { suit: "Souzu", rank: 4 }, { suit: "Souzu", rank: 6 }, { suit: "Souzu", rank: 8 }] },
  { id: "daisangen", name_zh: "大三元", base_fan: 8, desc: "白/發/中各一刻子",
    example: [{ suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 2 }, { suit: "Dragon", rank: 3 }] },
  { id: "shousuushii", name_zh: "小四喜", base_fan: 8, desc: "三种风刻子 + 一对风牌",
    example: [{ suit: "Wind", rank: 1 }, { suit: "Wind", rank: 2 }, { suit: "Wind", rank: 3 }, { suit: "Wind", rank: 4 }] },
  { id: "chinroutou", name_zh: "清老头", base_fan: 8, desc: "全部由1/9数牌组成(无字牌)",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 9 }, { suit: "Pinzu", rank: 1 }, { suit: "Souzu", rank: 9 }] },
  { id: "chuuren", name_zh: "九莲宝灯", base_fan: 8, desc: "同花色1112345678999+任一张",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 2 }, { suit: "Manzu", rank: 5 }] },
  { id: "kokushi", name_zh: "国士无双", base_fan: 8, desc: "13种么九牌各一 + 任一重复",
    example: [{ suit: "Manzu", rank: 1 }, { suit: "Manzu", rank: 9 }, { suit: "Pinzu", rank: 1 }, { suit: "Wind", rank: 1 }, { suit: "Dragon", rank: 3 }] },
  { id: "suuankou", name_zh: "四暗刻", base_fan: 8, desc: "四个暗刻",
    example: [{ suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 3 }, { suit: "Manzu", rank: 3 }, { suit: "Pinzu", rank: 7 }, { suit: "Pinzu", rank: 7 }] },
  { id: "daisuushii", name_zh: "大四喜", base_fan: 10, desc: "东南西北各一刻子",
    example: [{ suit: "Wind", rank: 1 }, { suit: "Wind", rank: 2 }, { suit: "Wind", rank: 3 }, { suit: "Wind", rank: 4 }] },
  { id: "suukantsu", name_zh: "四杠子", base_fan: 10, desc: "四个杠",
    example: [{ suit: "Manzu", rank: 5 }, { suit: "Manzu", rank: 5 }, { suit: "Manzu", rank: 5 }, { suit: "Manzu", rank: 5 }] },
  { id: "tsuuiisou", name_zh: "字一色", base_fan: 10, desc: "全部由字牌(风+三元)组成",
    example: [{ suit: "Wind", rank: 1 }, { suit: "Wind", rank: 2 }, { suit: "Dragon", rank: 1 }, { suit: "Dragon", rank: 2 }, { suit: "Dragon", rank: 3 }] },
];

export function PatternCodex({ onClose }: { onClose: () => void }) {
  return (
    <div className="codex-overlay" onClick={onClose}>
      <div className="codex-menu" onClick={(e) => e.stopPropagation()}>
        <h3 className="codex-title">牌型图鉴</h3>
        <div className="codex-grid">
          {PATTERNS.map((p) => (
            <div key={p.id} className="codex-card">
              <div className="codex-name">{p.name_zh}</div>
              <div className="codex-desc">{p.desc}</div>
              <div className="codex-example">
                {p.example.map((t, i) => (
                  <img
                    key={i}
                    className="codex-tile-img"
                    src={getTileImagePathFromSuitRank(t.suit, t.rank)}
                    alt={`${t.suit}${t.rank}`}
                  />
                ))}
                <span className="codex-ellipsis">...</span>
              </div>
              <div className="codex-fan">{p.base_fan}番</div>
            </div>
          ))}
        </div>
        <button className="btn btn-secondary" onClick={onClose}>关闭</button>
      </div>
    </div>
  );
}

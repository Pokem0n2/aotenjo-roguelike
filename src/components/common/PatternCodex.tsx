export function PatternCodex({ onClose }: { onClose: () => void }) {
  // Static pattern list — mirrors pattern_checker.rs
  const patterns = [
    { id: "tanyao", name_zh: "断么九", base_fan: 1, desc: "全部为中张牌(2-8)" },
    { id: "iipeikou", name_zh: "一杯口", base_fan: 1, desc: "两组相同的顺子" },
    { id: "yakuhai_haku", name_zh: "役牌·白", base_fan: 1, desc: "白板刻子" },
    { id: "yakuhai_hatsu", name_zh: "役牌·發", base_fan: 1, desc: "發财刻子" },
    { id: "yakuhai_chun", name_zh: "役牌·中", base_fan: 1, desc: "红中刻子" },
    { id: "yakuhai_wind", name_zh: "役牌·场风", base_fan: 1, desc: "场风刻子" },
    { id: "chanta", name_zh: "混全带么", base_fan: 2, desc: "每副面子含么九或字牌" },
    { id: "sanshoku", name_zh: "三色同顺", base_fan: 2, desc: "三种花色同数顺子" },
    { id: "ittsu", name_zh: "一气通贯", base_fan: 2, desc: "同花色1-9顺子" },
    { id: "toitoi", name_zh: "对对和", base_fan: 2, desc: "全部刻子" },
    { id: "sanankou", name_zh: "三暗刻", base_fan: 2, desc: "三个暗刻" },
    { id: "chiitoitsu", name_zh: "七对子", base_fan: 2, desc: "七个不同的对子" },
    { id: "honitsu", name_zh: "混一色", base_fan: 3, desc: "一种数牌+字牌" },
    { id: "junchan", name_zh: "纯全带么", base_fan: 3, desc: "每副面子含么九，无字牌" },
    { id: "ryanpeikou", name_zh: "二杯口", base_fan: 3, desc: "两对相同顺子" },
    { id: "sankantsu", name_zh: "三杠子", base_fan: 3, desc: "三个杠" },
    { id: "honroutou", name_zh: "混老头", base_fan: 3, desc: "全部么九+字牌" },
    { id: "shousangen", name_zh: "小三元", base_fan: 4, desc: "两种三元刻子+一对三元" },
    { id: "chinitsu", name_zh: "清一色", base_fan: 6, desc: "纯一种数牌" },
    { id: "ryuuiisou", name_zh: "绿一色", base_fan: 6, desc: "全绿牌(2,3,4,6,8索+發)" },
    { id: "daisangen", name_zh: "大三元", base_fan: 8, desc: "三种三元牌刻子" },
    { id: "shousuushii", name_zh: "小四喜", base_fan: 8, desc: "三种风刻子+一对风" },
    { id: "chinroutou", name_zh: "清老头", base_fan: 8, desc: "全部么九牌(无字)" },
    { id: "chuuren", name_zh: "九莲宝灯", base_fan: 8, desc: "同花色1112345678999+任一" },
    { id: "kokushi", name_zh: "国士无双", base_fan: 8, desc: "13种么九各一+任一" },
    { id: "suuankou", name_zh: "四暗刻", base_fan: 8, desc: "四个暗刻" },
    { id: "daisuushii", name_zh: "大四喜", base_fan: 10, desc: "四种风牌刻子" },
    { id: "suukantsu", name_zh: "四杠子", base_fan: 10, desc: "四个杠" },
    { id: "tsuuiisou", name_zh: "字一色", base_fan: 10, desc: "全部字牌" },
  ];

  return (
    <div className="codex-overlay" onClick={onClose}>
      <div className="codex-menu" onClick={(e) => e.stopPropagation()}>
        <h3 className="codex-title">牌型图鉴</h3>
        <div className="codex-grid">
          {patterns.map((p) => (
            <div key={p.id} className="codex-card">
              <div className="codex-name">{p.name_zh}</div>
              <div className="codex-desc">{p.desc}</div>
              <div className="codex-fan">{p.base_fan}番</div>
            </div>
          ))}
        </div>
        <button className="btn btn-secondary" onClick={onClose}>关闭</button>
      </div>
    </div>
  );
}

export function TutorialPopup({ onClose }: { onClose: () => void }) {
  return (
    <div className="codex-overlay" onClick={onClose}>
      <div className="tutorial-menu" onClick={(e) => e.stopPropagation()}>
        <h3 className="codex-title">游戏教程</h3>

        <div className="tutorial-section">
          <h4 className="tutorial-heading">基本目标</h4>
          <p>每局通过 <strong>4 次出牌</strong>累积得分，达到目标分数即可通关。<br/>共 16 局（东南西北 × 4 局）。</p>
        </div>

        <div className="tutorial-section">
          <h4 className="tutorial-heading">如何出牌</h4>
          <p>从手牌中选出合法的牌型组合：</p>
          <ul>
            <li><strong>5 张牌</strong>：面子 + 对子（顺子/刻子 ×1 + 对子 ×1）</li>
            <li><strong>6 张牌</strong>：以下任一组合：</li>
            <ul>
              <li>杠 + 对子（4张相同 + 2张相同）</li>
              <li>顺子 + 顺子（两组同花色连续3张）</li>
              <li>刻子 + 刻子（两组3张相同）</li>
              <li>顺子 + 刻子（一组连续3张 + 一组3张相同）</li>
              <li>对子 + 对子 + 对子（三组2张相同）</li>
            </ul>
          </ul>
          <p><strong>基础概念：</strong></p>
          <ul>
            <li><strong>顺子</strong>（3张）：同花色连续3张，如一二三万</li>
            <li><strong>刻子</strong>（3张）：3张相同的牌</li>
            <li><strong>杠</strong>（4张）：4张相同的牌</li>
            <li><strong>对子</strong>（2张）：2张相同的牌</li>
          </ul>
        </div>

        <div className="tutorial-section">
          <h4 className="tutorial-heading">计分规则</h4>
          <p>最终得分 = <strong>符(Fu) × 番(Fan) × 倍率</strong></p>
          <ul>
            <li><strong>符</strong>：数牌的符值等于牌面数字(1-9)，字牌统一为10，遗物可额外加符</li>
            <li><strong>番</strong>：由牌型（役）提供，如断么九+1番、清一色+6番</li>
            <li><strong>倍率</strong>：部分遗物提供乘法效果</li>
          </ul>
        </div>

        <div className="tutorial-section">
          <h4 className="tutorial-heading">特殊操作</h4>
          <ul>
            <li><strong>跳过</strong>：放弃本次出牌，获得 8 张额外手牌</li>
            <li><strong>商店</strong>：每局通关后进入，购买遗物和道具</li>
            <li><strong>Boss</strong>：每局有特殊 Boss 限制（减少出牌次数、税收等）</li>
          </ul>
        </div>

        <div className="tutorial-section">
          <h4 className="tutorial-heading">超杀奖励</h4>
          <p>得分超过目标 2 倍以上可获得额外金币奖励，倍率越高奖励越多！</p>
        </div>

        <button className="btn btn-primary" onClick={onClose}>
          知道了
        </button>
      </div>
    </div>
  );
}

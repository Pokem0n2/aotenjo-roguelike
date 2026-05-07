import { useGameStore } from "../../store/game-store";
import { startRun } from "../../api/game";

export function GameOver() {
  const currentWind = useGameStore((s) => s.currentWind);
  const currentRound = useGameStore((s) => s.currentRound);
  const currency = useGameStore((s) => s.currency);
  const artifacts = useGameStore((s) => s.artifacts);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLastPlayResult = useGameStore((s) => s.setLastPlayResult);

  const windNames: Record<string, string> = { East: "东", South: "南", West: "西", North: "北" };

  const handleRestart = async () => {
    const state = await startRun();
    setFromStateView(state);
    setLastPlayResult(null as unknown as import("../../store/game-store").PlayResultView);
  };

  return (
    <div className="game-over-screen">
      <h1 className="gameover-title">旅途结束</h1>
      <div className="gameover-stats">
        <div className="stat-row">
          <span>到达</span>
          <span>{windNames[currentWind] || currentWind}风 第{currentRound}局</span>
        </div>
        <div className="stat-row">
          <span>获得金币</span>
          <span>{currency}</span>
        </div>
        <div className="stat-row">
          <span>收集遗物</span>
          <span>{artifacts.length} 件</span>
        </div>
      </div>
      <button className="btn btn-start" onClick={handleRestart}>
        再来一局
      </button>
    </div>
  );
}

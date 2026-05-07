import { useGameStore } from "../../store/game-store";
import { startRun } from "../../api/game";

export function VictoryScreen() {
  const currency = useGameStore((s) => s.currency);
  const artifacts = useGameStore((s) => s.artifacts);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLastPlayResult = useGameStore((s) => s.setLastPlayResult);

  const handleRestart = async () => {
    const state = await startRun();
    setFromStateView(state);
    setLastPlayResult(null as unknown as import("../../store/game-store").PlayResultView);
  };

  return (
    <div className="victory-screen">
      <h1 className="victory-title">通关!</h1>
      <p className="victory-subtitle">青天井 — 无限之顶</p>
      <div className="victory-stats">
        <div className="stat-row">
          <span>总金币</span>
          <span>{currency.toLocaleString()}</span>
        </div>
        <div className="stat-row">
          <span>遗物</span>
          <span>{artifacts.length} 件</span>
        </div>
      </div>
      <button className="btn btn-start" onClick={handleRestart}>
        再来一局
      </button>
    </div>
  );
}

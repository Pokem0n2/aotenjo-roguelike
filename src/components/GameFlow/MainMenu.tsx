import { useGameStore } from "../../store/game-store";
import { startRun } from "../../api/game";

export function MainMenu() {
  const phase = useGameStore((s) => s.phase);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLoading = useGameStore((s) => s.setLoading);
  const setError = useGameStore((s) => s.setError);

  const handleStart = async () => {
    setLoading(true);
    try {
      const state = await startRun();
      setFromStateView(state);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  if (phase === "MainMenu") {
    return (
      <div className="main-menu">
        <h1 className="title">青天井</h1>
        <p className="subtitle">Aotenjo Roguelike</p>
        <p className="tagline">麻将 × 肉鸽 = 无限可能</p>
        <button className="btn btn-start" onClick={handleStart}>
          开始新旅途
        </button>
      </div>
    );
  }

  // DeckSelect — simplified for now
  return (
    <div className="deck-select">
      <h2>选择起始牌组</h2>
      <div className="deck-list">
        <button className="btn btn-deck" onClick={handleStart}>
          标准牌组
        </button>
      </div>
    </div>
  );
}

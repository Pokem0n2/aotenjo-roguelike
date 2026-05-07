import { useCallback, useEffect } from "react";
import { useGameStore } from "../../store/game-store";
import { selectTilesForPlay, submitPlay, getGameState, skipPlay } from "../../api/game";
import { HandDisplay } from "./HandDisplay";
import { ScoreDisplay } from "../Scoring/ScoreDisplay";
import { ArtifactBar } from "../Artifacts/ArtifactBar";
import { RoundResult } from "../GameFlow/RoundResult";
import { GameOver } from "../GameFlow/GameOver";
import { VictoryScreen } from "../GameFlow/VictoryScreen";

export function Board() {
  const {
    phase,
    currentWind,
    currentRound,
    currentPlay,
    maxPlays,
    handTiles,
    selectedTileIds,
    wallRemaining,
    roundScore,
    roundTarget,
    currency,
    artifacts,
    isLoading,
    lastPlayResult,
    error,
  } = useGameStore();

  const setSelectedTileIds = useGameStore((s) => s.setSelectedTileIds);
  const setLastPlayResult = useGameStore((s) => s.setLastPlayResult);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLoading = useGameStore((s) => s.setLoading);
  const setError = useGameStore((s) => s.setError);

  const windNames: Record<string, string> = { East: "东", South: "南", West: "西", North: "北" };
  const roundOver = currentPlay >= maxPlays || roundScore >= roundTarget;

  // Auto-detect round over → switch to RoundResult phase
  useEffect(() => {
    if (phase === "Playing" && roundOver && currentPlay > 0) {
      useGameStore.setState({ phase: "RoundResult" });
    }
  }, [phase, roundOver, currentPlay]);

  const refreshState = useCallback(async () => {
    const state = await getGameState();
    setFromStateView(state);
    setSelectedTileIds([]);
  }, [setFromStateView, setSelectedTileIds]);

  const handleTileClick = useCallback(
    async (tileId: number) => {
      const newSelected = selectedTileIds.includes(tileId)
        ? selectedTileIds.filter((id) => id !== tileId)
        : [...selectedTileIds, tileId];
      setSelectedTileIds(newSelected);
      try {
        await selectTilesForPlay(newSelected);
      } catch (e) {
        setError(String(e));
      }
    },
    [selectedTileIds, setSelectedTileIds, setError]
  );

  const handleSubmitPlay = useCallback(async () => {
    if (selectedTileIds.length < 5) {
      setError("请选择至少5张牌（一组面子 + 一对）");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const result = await submitPlay();
      setLastPlayResult(result);
      await refreshState();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [selectedTileIds, setLastPlayResult, refreshState, setLoading, setError]);

  const handleSkip = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const state = await skipPlay();
      setFromStateView(state);
      setSelectedTileIds([]);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [setFromStateView, setSelectedTileIds, setLoading, setError]);

  // Phase routing
  if (phase === "GameOver") return <GameOver />;
  if (phase === "Victory") return <VictoryScreen />;
  if (phase === "RoundResult") return <RoundResult />;

  return (
    <div className="board">
      <div className="board-topbar">
        <span className="wind-indicator">
          {windNames[currentWind] || currentWind}风 第{currentRound}局
        </span>
        <span className="play-counter">
          出牌 {currentPlay}/{maxPlays}
        </span>
        <span className="currency">💰 {currency}</span>
        <span className="wall-info">牌墙剩余: {wallRemaining}</span>
      </div>

      <ScoreDisplay
        score={roundScore}
        target={roundTarget}
        lastResult={lastPlayResult}
      />

      <ArtifactBar artifacts={artifacts} />

      <HandDisplay
        tiles={handTiles}
        selectedIds={selectedTileIds}
        onTileClick={handleTileClick}
      />

      <div className="board-actions">
        <button
          className="btn btn-primary"
          onClick={handleSubmitPlay}
          disabled={isLoading || selectedTileIds.length < 5}
        >
          {isLoading ? "计算中..." : "出牌"}
        </button>
        <button
          className="btn btn-secondary"
          onClick={handleSkip}
          disabled={isLoading || roundOver}
        >
          跳过(+8牌)
        </button>
        <span className="selected-count">
          已选 {selectedTileIds.length} 张
        </span>
      </div>

      {error && <div className="error-toast">{error}</div>}
    </div>
  );
}

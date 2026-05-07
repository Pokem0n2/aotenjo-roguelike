import { useGameStore } from "../../store/game-store";
import { endRound, startNextRound, getGameState } from "../../api/game";

export function RoundResult() {
  const roundScore = useGameStore((s) => s.roundScore);
  const roundTarget = useGameStore((s) => s.roundTarget);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLastPlayResult = useGameStore((s) => s.setLastPlayResult);
  const setError = useGameStore((s) => s.setError);

  const passed = roundScore >= roundTarget;
  const ratio = roundTarget > 0 ? roundScore / roundTarget : 0;
  const overkill = ratio >= 2;

  const handleContinue = async () => {
    try {
      const result = await endRound();
      if (result.victory) {
        const gs = await getGameState();
        setFromStateView(gs);
        return;
      }
      if (result.passed) {
        const gs = await startNextRound();
        setFromStateView(gs);
        setLastPlayResult(null as unknown as import("../../store/game-store").PlayResultView);
      } else {
        const gs = await getGameState();
        setFromStateView(gs);
      }
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div className="round-result">
      {passed ? (
        <>
          <h2 className="result-title result-pass">
            {overkill ? "超杀通关!" : "通关!"}
          </h2>
          <div className="result-score">
            {roundScore.toLocaleString()} / {roundTarget.toLocaleString()}
          </div>
          {overkill && (
            <div className="overkill-badge">
              ×{ratio.toFixed(1)} 超杀
            </div>
          )}
          <button className="btn btn-primary" onClick={handleContinue}>
            下一局 →
          </button>
        </>
      ) : (
        <>
          <h2 className="result-title result-fail">未达标</h2>
          <div className="result-score">
            {roundScore.toLocaleString()} / {roundTarget.toLocaleString()}
          </div>
          <button className="btn btn-primary" onClick={handleContinue}>
            查看结果
          </button>
        </>
      )}
    </div>
  );
}

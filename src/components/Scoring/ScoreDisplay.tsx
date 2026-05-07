import type { PlayResultView } from "../../store/game-store";

interface ScoreDisplayProps {
  score: number;
  target: number;
  lastResult: PlayResultView | null;
}

export function ScoreDisplay({ score, target, lastResult }: ScoreDisplayProps) {
  const progress = target > 0 ? Math.min((score / target) * 100, 100) : 0;
  const isOver = score >= target;

  return (
    <div className="score-display">
      <div className="score-bar">
        <div className="score-labels">
          <span className="score-current">
            {score.toLocaleString()}
          </span>
          <span className="score-separator">/</span>
          <span className="score-target">
            {target.toLocaleString()}
          </span>
        </div>
        <div className="progress-bar">
          <div
            className={`progress-fill ${isOver ? "progress-fill--over" : ""}`}
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>
      {lastResult && (
        <div className="last-score">
          +{lastResult.score.toLocaleString()}
        </div>
      )}
    </div>
  );
}

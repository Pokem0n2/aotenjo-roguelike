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
      <div className="score-main">
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
        <div className="score-detail">
          <div className="score-formula">
            {lastResult.fu}符 × {lastResult.fan.toFixed(1)}番
            {lastResult.mult > 1.0 && ` × ${lastResult.mult.toFixed(1)}`}
            {" = "}
            <strong>{lastResult.score.toLocaleString()}</strong>
          </div>
          {lastResult.patterns.length > 0 && (
            <div className="score-patterns">
              {lastResult.patterns.map((p, i) => (
                <span key={i} className="pattern-tag">{p}</span>
              ))}
            </div>
          )}
          {lastResult.breakdown.length > 0 && (
            <details className="score-breakdown">
              <summary>计分明细</summary>
              {lastResult.breakdown.map((line, i) => (
                <div key={i} className="breakdown-line">{line}</div>
              ))}
            </details>
          )}
        </div>
      )}
    </div>
  );
}

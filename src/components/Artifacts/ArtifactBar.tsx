import type { ArtifactView } from "../../store/game-store";

interface ArtifactBarProps {
  artifacts: ArtifactView[];
}

const RARITY_COLORS: Record<string, string> = {
  Common: "#9e9e9e",
  Uncommon: "#4caf50",
  Rare: "#2196f3",
  Legendary: "#ff9800",
};

const RARITY_ZH: Record<string, string> = {
  Common: "普通",
  Uncommon: "稀有",
  Rare: "珍贵",
  Legendary: "传说",
};

export function ArtifactBar({ artifacts }: ArtifactBarProps) {
  if (artifacts.length === 0) {
    return (
      <div className="artifact-bar artifact-bar--empty">
        <span className="artifact-bar-label">遗物</span>
        <span className="artifact-empty">暂无遗物 — 在商店中购买</span>
      </div>
    );
  }

  return (
    <div className="artifact-bar">
      <span className="artifact-bar-label">遗物 ({artifacts.length})</span>
      <div className="artifact-list">
        {artifacts.map((artifact) => (
          <div
            key={artifact.id}
            className="artifact-slot"
            style={{ borderColor: RARITY_COLORS[artifact.rarity] || "#666" }}
            title={`${artifact.name_zh}\n${artifact.description_zh}\n[${RARITY_ZH[artifact.rarity] || artifact.rarity}]`}
          >
            <span
              className="artifact-icon"
              style={{ color: RARITY_COLORS[artifact.rarity] || "#666" }}
            >
              {artifact.name_zh[0]}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

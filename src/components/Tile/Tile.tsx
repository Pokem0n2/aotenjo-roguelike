import type { TileView } from "../../store/game-store";

interface TileProps {
  tile: TileView;
  selected?: boolean;
  onClick?: () => void;
}

// 每种花色统一颜色：万=红、索=绿、筒=黑
const SUIT_CONFIG: Record<string, { char: string; color: string }> = {
  Manzu:  { char: "万", color: "#c62828" }, // 红色
  Pinzu:  { char: "筒", color: "#212121" }, // 黑色
  Souzu:  { char: "索", color: "#2e7d32" }, // 绿色
  Wind:   { char: "",   color: "#37474f" },
  Dragon: { char: "",   color: "#37474f" },
};

const WIND_CHARS = ["东", "南", "西", "北"];
const DRAGON_CHARS = ["白", "發", "中"];
const DRAGON_COLORS = ["#757575", "#2e7d32", "#c62828"];
const NUM_CHARS = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];

function getTileDisplay(tile: TileView): { top: string; bottom: string; color: string } {
  switch (tile.suit) {
    case "Manzu":
    case "Pinzu":
    case "Souzu": {
      const cfg = SUIT_CONFIG[tile.suit];
      return {
        top: NUM_CHARS[tile.rank - 1],
        bottom: cfg.char,
        color: cfg.color,
      };
    }
    case "Wind":
      return {
        top: WIND_CHARS[tile.rank - 1],
        bottom: "",
        color: SUIT_CONFIG.Wind.color,
      };
    case "Dragon":
      return {
        top: DRAGON_CHARS[tile.rank - 1],
        bottom: "",
        color: DRAGON_COLORS[tile.rank - 1],
      };
    default:
      return { top: "?", bottom: "", color: "#333" };
  }
}

export function Tile({ tile, selected, onClick }: TileProps) {
  const display = getTileDisplay(tile);

  return (
    <div
      className={`tile ${selected ? "tile--selected" : ""}`}
      onClick={onClick}
      title={tile.display_zh}
    >
      <div className="tile-inner">
        <span className="tile-top" style={{ color: display.color }}>
          {display.top}
        </span>
        {display.bottom && (
          <span className="tile-bottom" style={{ color: display.color }}>
            {display.bottom}
          </span>
        )}
      </div>
    </div>
  );
}

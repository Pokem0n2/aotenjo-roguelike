import type { TileView } from "../../store/game-store";

interface TileProps {
  tile: TileView;
  selected?: boolean;
  onClick?: () => void;
}

const SUIT_SYMBOLS: Record<string, { char: string; color: string }> = {
  Manzu: { char: "万", color: "#1565c0" },
  Pinzu: { char: "筒", color: "#1b5e20" },
  Souzu: { char: "索", color: "#2e7d32" },
  Wind: { char: "", color: "#37474f" },
  Dragon: { char: "", color: "#b71c1c" },
};

const WIND_CHARS = ["东", "南", "西", "北"];
const DRAGON_CHARS = ["白", "發", "中"];
const NUM_CHARS = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];

function getTileDisplay(tile: TileView): { top: string; bottom: string; color: string } {
  const suitInfo = SUIT_SYMBOLS[tile.suit] || { char: "", color: "#333" };

  switch (tile.suit) {
    case "Manzu":
    case "Pinzu":
    case "Souzu": {
      const numChar = NUM_CHARS[tile.rank - 1];
      return {
        top: numChar,
        bottom: suitInfo.char,
        color: tile.is_red ? "#c62828" : suitInfo.color,
      };
    }
    case "Wind":
      return {
        top: WIND_CHARS[tile.rank - 1],
        bottom: "風",
        color: "#37474f",
      };
    case "Dragon": {
      const colors = ["#9e9e9e", "#2e7d32", "#c62828"];
      return {
        top: DRAGON_CHARS[tile.rank - 1],
        bottom: "",
        color: colors[tile.rank - 1],
      };
    }
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

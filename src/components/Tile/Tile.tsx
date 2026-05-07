import type { TileView } from "../../store/game-store";

interface TileProps {
  tile: TileView;
  selected?: boolean;
  onClick?: () => void;
}

function getTileImagePath(tile: TileView): string {
  switch (tile.suit) {
    case "Manzu":  return `/tiles/${tile.rank}m.png`;
    case "Pinzu":  return `/tiles/${tile.rank}p.png`;
    case "Souzu":  return `/tiles/${tile.rank}s.png`;
    case "Wind":   return `/tiles/${tile.rank}z.png`;  // 1z-4z = 东南西北
    case "Dragon": return `/tiles/${tile.rank + 4}z.png`; // 5z=白, 6z=發, 7z=中
    default:       return "";
  }
}

export function Tile({ tile, selected, onClick }: TileProps) {
  const imgPath = getTileImagePath(tile);

  return (
    <div
      className={`tile ${selected ? "tile--selected" : ""}`}
      onClick={onClick}
      title={tile.display_zh}
    >
      <img
        className="tile-image"
        src={imgPath}
        alt={tile.display_zh}
        draggable={false}
      />
    </div>
  );
}

export function TileImage({ tile, size = 32 }: { tile: TileView; size?: number }) {
  const imgPath = getTileImagePath(tile);
  return (
    <img
      className="tile-inline-image"
      src={imgPath}
      alt={tile.display_zh}
      style={{ width: size, height: size * 1.38 }}
      draggable={false}
    />
  );
}

export function getTileImagePathFromSuitRank(suit: string, rank: number): string {
  switch (suit) {
    case "Manzu":  return `/tiles/${rank}m.png`;
    case "Pinzu":  return `/tiles/${rank}p.png`;
    case "Souzu":  return `/tiles/${rank}s.png`;
    case "Wind":   return `/tiles/${rank}z.png`;
    case "Dragon": return `/tiles/${rank + 4}z.png`;
    default:       return "";
  }
}

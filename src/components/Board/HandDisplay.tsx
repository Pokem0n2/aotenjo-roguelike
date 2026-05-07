import { Tile } from "../Tile/Tile";
import type { TileView } from "../../store/game-store";

interface HandDisplayProps {
  tiles: TileView[];
  selectedIds: number[];
  onTileClick: (tileId: number) => void;
}

export function HandDisplay({ tiles, selectedIds, onTileClick }: HandDisplayProps) {
  return (
    <div className="hand-display">
      <div className="hand-label">手牌</div>
      <div className="hand-tiles">
        {tiles.map((tile) => (
          <Tile
            key={tile.id}
            tile={tile}
            selected={selectedIds.includes(tile.id)}
            onClick={() => onTileClick(tile.id)}
          />
        ))}
      </div>
    </div>
  );
}

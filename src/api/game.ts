import { invoke } from "@tauri-apps/api/core";
import type { GameStateView, PlayResultView, TileView } from "../store/game-store";

export async function startRun(seed?: number): Promise<GameStateView> {
  return invoke("start_run", { seed: seed ?? null });
}

export async function drawTiles(count: number): Promise<TileView[]> {
  return invoke("draw_tiles", { count });
}

export async function getGameState(): Promise<GameStateView> {
  return invoke("get_game_state");
}

export async function selectTilesForPlay(tileIds: number[]): Promise<TileView[]> {
  return invoke("select_tiles_for_play", { tileIds });
}

export async function submitPlay(): Promise<PlayResultView> {
  return invoke("submit_play");
}

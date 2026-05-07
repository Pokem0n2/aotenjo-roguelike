import { invoke } from "@tauri-apps/api/core";
import type { GameStateView, PlayResultView, TileView } from "../store/game-store";

export interface EndRoundView {
  passed: boolean;
  score: number;
  target: number;
  bonus: number;
  victory: boolean;
}

export interface GadgetUseResult {
  message: string;
  game_state: GameStateView;
}

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

export async function endRound(): Promise<EndRoundView> {
  return invoke("end_round");
}

export async function skipPlay(): Promise<GameStateView> {
  return invoke("skip_play");
}

export async function startNextRound(): Promise<GameStateView> {
  return invoke("start_next_round");
}

// Shop operations
export async function shopBuy(index: number): Promise<GameStateView> {
  return invoke("shop_buy", { index });
}

export async function shopSellArtifact(artifactId: string): Promise<GameStateView> {
  return invoke("shop_sell_artifact", { artifactId });
}

export async function shopSellGadget(gadgetId: string): Promise<GameStateView> {
  return invoke("shop_sell_gadget", { gadgetId });
}

export async function shopReroll(): Promise<GameStateView> {
  return invoke("shop_reroll");
}

export async function shopLeave(): Promise<GameStateView> {
  return invoke("shop_leave");
}

export async function useGadget(gadgetId: string, tileIds: number[]): Promise<GadgetUseResult> {
  return invoke("use_gadget", { gadgetId, tileIds });
}

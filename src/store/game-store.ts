import { create } from "zustand";

export interface TileView {
  suit: string;
  rank: number;
  id: number;
  is_red: boolean;
  display_zh: string;
  base_fu: number;
}

export interface ArtifactView {
  id: string;
  name_zh: string;
  name_en: string;
  description_zh: string;
  rarity: string;
  sell_value: number;
}

export interface GameStateView {
  phase: string;
  current_wind: string;
  current_round: number;
  current_play: number;
  max_plays: number;
  hand_tiles: TileView[];
  wall_remaining: number;
  round_score: number;
  round_target: number;
  currency: number;
  artifacts: ArtifactView[];
}

export interface PlayResultView {
  score: number;
  round_score: number;
  round_target: number;
  round_over: boolean;
  fu: number;
  fan: number;
  mult: number;
  patterns: string[];
  breakdown: string[];
}

interface GameStore {
  phase: string;
  currentWind: string;
  currentRound: number;
  currentPlay: number;
  maxPlays: number;
  handTiles: TileView[];
  selectedTileIds: number[];
  wallRemaining: number;
  roundScore: number;
  roundTarget: number;
  currency: number;
  artifacts: ArtifactView[];
  lastPlayResult: PlayResultView | null;
  isLoading: boolean;
  error: string | null;

  setFromStateView: (view: GameStateView) => void;
  setSelectedTileIds: (ids: number[]) => void;
  setLastPlayResult: (result: PlayResultView) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useGameStore = create<GameStore>((set) => ({
  phase: "MainMenu",
  currentWind: "East",
  currentRound: 1,
  currentPlay: 0,
  maxPlays: 4,
  handTiles: [],
  selectedTileIds: [],
  wallRemaining: 0,
  roundScore: 0,
  roundTarget: 0,
  currency: 0,
  artifacts: [],
  lastPlayResult: null,
  isLoading: false,
  error: null,

  setFromStateView: (view) =>
    set({
      phase: view.phase,
      currentWind: view.current_wind,
      currentRound: view.current_round,
      currentPlay: view.current_play,
      maxPlays: view.max_plays,
      handTiles: view.hand_tiles,
      wallRemaining: view.wall_remaining,
      roundScore: view.round_score,
      roundTarget: view.round_target,
      currency: view.currency,
      artifacts: view.artifacts,
    }),

  setSelectedTileIds: (ids) => set({ selectedTileIds: ids }),
  setLastPlayResult: (result) => set({ lastPlayResult: result }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
}));

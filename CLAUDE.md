# Aotenjo Roguelike (青天井)

A riichi mahjong roguelike game inspired by Steam's "Aotenjo: Infinite Hands".

## Tech Stack

- **Tauri 2.x** — Desktop app framework (Rust backend + React frontend)
- **React 19 + TypeScript** — Frontend UI
- **Vite** — Build tool
- **Rust** — All game logic (state, scoring, pattern validation)
- **pnpm** — Package manager

## Development

```bash
cd aotenjo-roguelike
pnpm install
pnpm tauri dev
```

## Build

```bash
pnpm tauri build
```

Output: `src-tauri/target/release/bundle/nsis/`

## Architecture

### Rust Backend (`src-tauri/src/`)

- `models/` — Core data types: Tile, Wall, Hand, Pattern, Artifact, Gadget, Deck, Boss
- `game/` — Game logic: state machine, pattern checker, scoring engine
- `commands/` — Tauri IPC command handlers
- `persistence/` — Save/load system

### React Frontend (`src/`)

- `components/` — UI components: Tile, Board, Scoring, Shop, Artifacts, GameFlow
- `store/` — Zustand state management
- `api/` — Tauri IPC wrappers

## Game Mechanics

- **Fu × Fan** scoring model (Fan from patterns/yaku, Fu from artifacts/tile buffs)
- 16 rounds across 4 winds (East, South, West, North)
- 4 plays per round, cumulative scoring
- 102+ patterns (yaku), 185+ artifacts, gadgets, tile upgrades
- Boss encounters with unique gimmicks
- Overkill economy (Aotenjo Bonus)

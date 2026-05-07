import { useGameStore } from "./store/game-store";
import { Board } from "./components/Board/Board";
import { MainMenu } from "./components/GameFlow/MainMenu";
import "./App.css";

function App() {
  const phase = useGameStore((s) => s.phase);

  if (phase === "MainMenu" || phase === "DeckSelect") {
    return (
      <div className="app">
        <MainMenu />
      </div>
    );
  }

  // Playing, RoundResult, Shop, GameOver, Victory — all handled by Board
  return (
    <div className="app">
      <Board />
    </div>
  );
}

export default App;

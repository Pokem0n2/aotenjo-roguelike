import { useGameStore } from "./store/game-store";
import { Board } from "./components/Board/Board";
import { MainMenu } from "./components/GameFlow/MainMenu";
import "./App.css";

function App() {
  const phase = useGameStore((s) => s.phase);

  return (
    <div className="app">
      {phase === "MainMenu" || phase === "DeckSelect" ? (
        <MainMenu />
      ) : (
        <Board />
      )}
    </div>
  );
}

export default App;

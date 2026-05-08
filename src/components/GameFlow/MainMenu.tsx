import { useState, useEffect, useCallback } from "react";
import { useGameStore } from "../../store/game-store";
import { startRun, loadGame, listSaves, deleteSave } from "../../api/game";
import type { SaveMeta } from "../../api/game";
import { SettingsMenu } from "../common/SettingsMenu";
import { PatternCodex } from "../common/PatternCodex";
import { TutorialPopup } from "../common/TutorialPopup";

export function MainMenu() {
  const phase = useGameStore((s) => s.phase);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setLoading = useGameStore((s) => s.setLoading);
  const setError = useGameStore((s) => s.setError);
  const isLoading = useGameStore((s) => s.isLoading);

  const [showSaves, setShowSaves] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showCodex, setShowCodex] = useState(false);
  const [showTutorial, setShowTutorial] = useState(false);
  const [saves, setSaves] = useState<SaveMeta[]>([]);

  useEffect(() => {
    if (showSaves) {
      listSaves().then(setSaves).catch(() => {});
    }
  }, [showSaves]);

  const handleStart = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const state = await startRun();
      setFromStateView(state);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [setFromStateView, setLoading, setError]);

  const handleLoad = useCallback(async (slot: number) => {
    setLoading(true);
    setError(null);
    try {
      const state = await loadGame(slot);
      setFromStateView(state);
      setShowSaves(false);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [setFromStateView, setLoading, setError]);

  const handleDeleteSave = useCallback(async (slot: number) => {
    try {
      await deleteSave(slot);
      const updated = await listSaves();
      setSaves(updated);
    } catch (e) {
      setError(String(e));
    }
  }, [setError]);

  const windNames: Record<string, string> = { East: "东", South: "南", West: "西", North: "北" };

  if (phase === "MainMenu") {
    return (
      <div className="main-menu">
        <h1 className="title">青天井</h1>
        <p className="subtitle">Aotenjo Roguelike</p>
        <p className="tagline">麻将 × 肉鸽 = 无限可能</p>
        <div className="main-menu-buttons">
          <button className="btn btn-start" onClick={handleStart} disabled={isLoading}>
            {isLoading ? "加载中..." : "开始新旅途"}
          </button>
        </div>

        <div className="main-menu-footer">
          <button className="btn btn-sm btn-secondary" onClick={() => setShowSaves(!showSaves)}>
            {showSaves ? "关闭存档" : "读取存档"}
          </button>
          <button className="btn btn-sm btn-secondary" onClick={() => setShowTutorial(true)}>
            游戏教程
          </button>
          <button className="btn btn-sm btn-secondary" onClick={() => setShowCodex(true)}>
            牌型图鉴
          </button>
          <button className="btn btn-sm btn-secondary" onClick={() => setShowSettings(true)}>
            设置
          </button>
        </div>

        {showSaves && (
          <div className="save-slots">
            {[0, 1, 2].map((slot) => {
              const save = saves.find((s) => s.slot === slot);
              return (
                <div key={slot} className="save-slot">
                  <div className="save-slot-info">
                    <span className="save-slot-label">存档 {slot + 1}</span>
                    {save ? (
                      <span className="save-slot-detail">
                        {windNames[save.wind] || save.wind}风 第{save.round}局 · 💰{save.currency}
                      </span>
                    ) : (
                      <span className="save-slot-empty">空</span>
                    )}
                  </div>
                  <div className="save-slot-actions">
                    {save && (
                      <>
                        <button className="btn btn-sm btn-primary" onClick={() => handleLoad(slot)}>
                          读取
                        </button>
                        <button className="btn btn-sm btn-danger" onClick={() => handleDeleteSave(slot)}>
                          删除
                        </button>
                      </>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        )}

        {showSettings && <SettingsMenu onClose={() => setShowSettings(false)} />}
        {showCodex && <PatternCodex onClose={() => setShowCodex(false)} />}
        {showTutorial && <TutorialPopup onClose={() => setShowTutorial(false)} />}
      </div>
    );
  }

  // DeckSelect — simplified
  return (
    <div className="deck-select">
      <h2>选择起始牌组</h2>
      <div className="deck-list">
        <button className="btn btn-deck" onClick={handleStart}>
          标准牌组
        </button>
      </div>
    </div>
  );
}

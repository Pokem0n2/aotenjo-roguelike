import { useCallback, useState } from "react";
import { useGameStore } from "../../store/game-store";
import {
  shopBuy,
  shopReroll,
  shopLeave,
  shopSellArtifact,
  shopSellGadget,
} from "../../api/game";
import { ArtifactBar } from "../Artifacts/ArtifactBar";

export function ShopScreen() {
  const currency = useGameStore((s) => s.currency);
  const shopItems = useGameStore((s) => s.shopItems);
  const shopRerolls = useGameStore((s) => s.shopRerolls);
  const artifacts = useGameStore((s) => s.artifacts);
  const gadgets = useGameStore((s) => s.gadgets);
  const setFromStateView = useGameStore((s) => s.setFromStateView);
  const setError = useGameStore((s) => s.setError);
  const setLoading = useGameStore((s) => s.setLoading);

  const [sellMode, setSellMode] = useState(false);
  const [selectedSellId, setSelectedSellId] = useState<string | null>(null);
  const [selectedSellType, setSelectedSellType] = useState<
    "artifact" | "gadget" | null
  >(null);

  const handleBuy = useCallback(
    async (index: number) => {
      setLoading(true);
      setError(null);
      try {
        const state = await shopBuy(index);
        setFromStateView(state);
      } catch (e) {
        setError(String(e));
      } finally {
        setLoading(false);
      }
    },
    [setFromStateView, setLoading, setError]
  );

  const handleReroll = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const state = await shopReroll();
      setFromStateView(state);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [setFromStateView, setLoading, setError]);

  const handleLeave = useCallback(async () => {
    setLoading(true);
    try {
      const state = await shopLeave();
      setFromStateView(state);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [setFromStateView, setLoading, setError]);

  const handleConfirmSell = useCallback(async () => {
    if (!selectedSellId || !selectedSellType) return;
    setLoading(true);
    setError(null);
    try {
      let state;
      if (selectedSellType === "artifact") {
        state = await shopSellArtifact(selectedSellId);
      } else {
        state = await shopSellGadget(selectedSellId);
      }
      setFromStateView(state);
      setSelectedSellId(null);
      setSelectedSellType(null);
      setSellMode(false);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [selectedSellId, selectedSellType, setFromStateView, setLoading, setError]);

  const handleSellSelect = (id: string, type: "artifact" | "gadget") => {
    setSelectedSellId(id);
    setSelectedSellType(type);
  };

  const rarityColor: Record<string, string> = {
    Common: "#888",
    Uncommon: "#4caf50",
    Rare: "#42a5f5",
    Legendary: "#ff9800",
  };

  return (
    <div className="shop-screen">
      <div className="shop-header">
        <h2 className="shop-title">商店</h2>
        <span className="shop-currency">💰 {currency}</span>
      </div>

      <ArtifactBar artifacts={artifacts} />

      {/* Gadget inventory */}
      {gadgets.length > 0 && (
        <div className="gadget-bar">
          <span className="gadget-bar-label">道具:</span>
          <div className="gadget-list">
            {gadgets.map((g) => (
              <div
                key={g.id}
                className={`gadget-slot ${sellMode && selectedSellId === g.id ? "gadget-slot--selected" : ""}`}
                onClick={() =>
                  sellMode ? handleSellSelect(g.id, "gadget") : undefined
                }
                title={g.description_zh}
              >
                <span className="gadget-icon">{g.name_zh[0]}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Sell mode */}
      <div className="shop-sell-section">
        <button
          className={`btn ${sellMode ? "btn-danger" : "btn-secondary"}`}
          onClick={() => {
            setSellMode(!sellMode);
            setSelectedSellId(null);
          }}
        >
          {sellMode ? "取消出售" : "出售遗物/道具"}
        </button>
        {sellMode && selectedSellId && (
          <button className="btn btn-primary" onClick={handleConfirmSell}>
            确认出售
          </button>
        )}
      </div>

      {/* Shop items grid */}
      <div className="shop-grid">
        {shopItems.map((item) => (
          <div
            key={item.index}
            className={`shop-card ${item.sold ? "shop-card--sold" : ""}`}
            style={{
              borderColor: item.rarity
                ? rarityColor[item.rarity] || "#666"
                : "#9c27b0",
            }}
          >
            <div className="shop-card-type">
              {item.item_type === "artifact" ? "遗物" : "道具"}
            </div>
            <div className="shop-card-name">{item.name_zh}</div>
            <div className="shop-card-desc">{item.description_zh}</div>
            {item.rarity && (
              <div
                className="shop-card-rarity"
                style={{ color: rarityColor[item.rarity] || "#fff" }}
              >
                {item.rarity}
              </div>
            )}
            <div className="shop-card-cost">💰 {item.cost}</div>
            {!item.sold && (
              <button
                className="btn btn-shop-buy"
                onClick={() => handleBuy(item.index)}
                disabled={currency < item.cost}
              >
                {currency < item.cost ? "金币不足" : "购买"}
              </button>
            )}
            {item.sold && <div className="shop-card-sold-tag">已购</div>}
          </div>
        ))}
      </div>

      {/* Reroll + Leave */}
      <div className="shop-footer">
        <button
          className="btn btn-secondary"
          onClick={handleReroll}
          disabled={shopRerolls === 0}
        >
          重抽 ({shopRerolls}次) — 20💰
        </button>
        <button className="btn btn-primary" onClick={handleLeave}>
          下一局 →
        </button>
      </div>
    </div>
  );
}

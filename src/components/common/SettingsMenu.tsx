import { useState } from "react";
import { useTranslation } from "react-i18next";
import { setLanguage, LANGUAGES } from "../../i18n";

export function SettingsMenu({ onClose }: { onClose: () => void }) {
  const { t, i18n } = useTranslation();
  const [currentLang, setCurrentLang] = useState(i18n.language);

  const handleChangeLang = (code: string) => {
    setLanguage(code);
    setCurrentLang(code);
  };

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-menu" onClick={(e) => e.stopPropagation()}>
        <h3 className="settings-title">{t("settings", "设置")}</h3>

        <div className="settings-section">
          <label className="settings-label">Language / 语言</label>
          <div className="settings-lang-buttons">
            {LANGUAGES.map((lang) => (
              <button
                key={lang.code}
                className={`btn btn-sm ${currentLang === lang.code ? "btn-primary" : "btn-secondary"}`}
                onClick={() => handleChangeLang(lang.code)}
              >
                {lang.label}
              </button>
            ))}
          </div>
        </div>

        <button className="btn btn-secondary" onClick={onClose}>
          OK
        </button>
      </div>
    </div>
  );
}

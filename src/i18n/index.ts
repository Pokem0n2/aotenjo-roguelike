import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import zhCN from "./zh-CN.json";
import en from "./en.json";
import ja from "./ja.json";

const savedLang = localStorage.getItem("aotenjo-lang") || "zh-CN";

i18n.use(initReactI18next).init({
  resources: {
    "zh-CN": { translation: zhCN },
    en: { translation: en },
    ja: { translation: ja },
  },
  lng: savedLang,
  fallbackLng: "zh-CN",
  interpolation: { escapeValue: false },
});

export default i18n;

export function setLanguage(lang: string) {
  localStorage.setItem("aotenjo-lang", lang);
  i18n.changeLanguage(lang);
}

export const LANGUAGES = [
  { code: "zh-CN", label: "中文" },
  { code: "en", label: "English" },
  { code: "ja", label: "日本語" },
];

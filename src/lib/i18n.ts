import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";

import en from "../locales/en.json";
import zhCN from "../locales/zh-CN.json";

// Get system locale from Rust backend
async function getSystemLocale(): Promise<string> {
  try {
    const locale = await invoke<string>("get_system_locale");
    return locale;
  } catch {
    return "en-US";
  }
}

// Determine the language to use
function detectLanguage(locale: string): string {
  const normalizedLocale = locale.toLowerCase();
  
  if (normalizedLocale.startsWith("zh")) {
    return "zh-CN";
  }
  
  return "en";
}

// Initialize i18n
async function initI18n() {
  const systemLocale = await getSystemLocale();
  const language = detectLanguage(systemLocale);

  await i18n.use(initReactI18next).init({
    resources: {
      en: { translation: en },
      "zh-CN": { translation: zhCN },
    },
    lng: language,
    fallbackLng: "en",
    interpolation: {
      escapeValue: false,
    },
  });
}

// Initialize synchronously with English as default, then update
i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    "zh-CN": { translation: zhCN },
  },
  lng: "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
});

// Update language asynchronously
initI18n().catch(console.error);

export default i18n;

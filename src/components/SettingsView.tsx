import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useTranslation } from "react-i18next";

interface Settings {
  min_year: number;
  max_year: number;
  display_duration_ms: number;
  time_format: string;
}

const TIME_FORMATS = [
  { value: "%Y-%m-%d %H:%M:%S", label: "YYYY-MM-DD HH:mm:ss" },
  { value: "%Y/%m/%d %H:%M:%S", label: "YYYY/MM/DD HH:mm:ss" },
  { value: "%d-%m-%Y %H:%M:%S", label: "DD-MM-YYYY HH:mm:ss" },
  { value: "%m/%d/%Y %H:%M:%S", label: "MM/DD/YYYY HH:mm:ss" },
  { value: "%Y-%m-%d", label: "YYYY-MM-DD" },
  { value: "%H:%M:%S", label: "HH:mm:ss" },
];

export default function SettingsView() {
  const { t } = useTranslation();
  const [settings, setSettings] = useState<Settings>({
    min_year: 1970,
    max_year: 2100,
    display_duration_ms: 3000,
    time_format: "%Y-%m-%d %H:%M:%S",
  });
  const [autostart, setAutostart] = useState(false);
  const [saving, setSaving] = useState(false);

  // Load settings on mount
  useEffect(() => {
    const loadSettings = async () => {
      try {
        const loaded = await invoke<Settings>("load_settings");
        setSettings(loaded);
      } catch (error) {
        console.error("Failed to load settings:", error);
      }

      try {
        const enabled = await isEnabled();
        setAutostart(enabled);
      } catch (error) {
        console.error("Failed to check autostart:", error);
      }
    };

    loadSettings();
  }, []);

  // Save settings
  const saveSettings = async () => {
    setSaving(true);
    try {
      await invoke("save_settings", {
        min_year: settings.min_year,
        max_year: settings.max_year,
        display_duration_ms: settings.display_duration_ms,
        time_format: settings.time_format,
      });
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
    setSaving(false);
  };

  // Toggle autostart
  const toggleAutostart = async () => {
    try {
      if (autostart) {
        await disable();
        setAutostart(false);
      } else {
        await enable();
        setAutostart(true);
      }
    } catch (error) {
      console.error("Failed to toggle autostart:", error);
    }
  };

  const handleChange = (field: keyof Settings, value: string | number) => {
    setSettings((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <div className="h-full bg-background p-6 overflow-auto">
      <h1 className="text-2xl font-semibold text-foreground mb-6">
        {t("settings.title")}
      </h1>

      {/* General Section */}
      <section className="mb-8">
        <h2 className="text-lg font-medium text-foreground mb-4">
          {t("settings.general")}
        </h2>

        {/* Autostart */}
        <div className="flex items-center justify-between py-3 border-b border-border">
          <div>
            <label className="text-sm font-medium text-foreground">
              {t("settings.launchAtLogin")}
            </label>
            <p className="text-xs text-muted-foreground mt-0.5">
              {t("settings.launchAtLoginDesc")}
            </p>
          </div>
          <button
            onClick={toggleAutostart}
            className={`
              relative w-11 h-6 rounded-full transition-colors
              ${autostart ? "bg-blue-500" : "bg-gray-300 dark:bg-gray-600"}
            `}
          >
            <span
              className={`
                absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform
                ${autostart ? "translate-x-5" : "translate-x-0"}
              `}
            />
          </button>
        </div>

        {/* Display Duration */}
        <div className="py-3 border-b border-border">
          <label className="text-sm font-medium text-foreground">
            {t("settings.displayDuration")}
          </label>
          <p className="text-xs text-muted-foreground mt-0.5 mb-2">
            {(settings.display_duration_ms / 1000).toFixed(1)}s
          </p>
          <input
            type="range"
            min="1500"
            max="10000"
            step="500"
            value={settings.display_duration_ms}
            onChange={(e) => handleChange("display_duration_ms", parseInt(e.target.value))}
            className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer"
          />
        </div>

        {/* Time Format */}
        <div className="py-3 border-b border-border">
          <label className="text-sm font-medium text-foreground">
            {t("settings.timeFormat")}
          </label>
          <select
            value={settings.time_format}
            onChange={(e) => handleChange("time_format", e.target.value)}
            className="
              mt-2 w-full px-3 py-2 rounded-md
              bg-muted text-foreground
              border border-border
              focus:outline-none focus:ring-2 focus:ring-blue-500
            "
          >
            {TIME_FORMATS.map((format) => (
              <option key={format.value} value={format.value}>
                {format.label}
              </option>
            ))}
          </select>
        </div>
      </section>

      {/* Filter Section */}
      <section className="mb-8">
        <h2 className="text-lg font-medium text-foreground mb-4">
          {t("settings.filter")}
        </h2>

        {/* Year Range */}
        <div className="py-3">
          <label className="text-sm font-medium text-foreground">
            {t("settings.yearRange")}
          </label>
          <p className="text-xs text-muted-foreground mt-0.5 mb-2">
            {t("settings.yearRangeDesc")}
          </p>
          <div className="flex items-center gap-3">
            <input
              type="number"
              min="1900"
              max="2200"
              value={settings.min_year}
              onChange={(e) => handleChange("min_year", parseInt(e.target.value))}
              className="
                w-24 px-3 py-2 rounded-md
                bg-muted text-foreground
                border border-border
                focus:outline-none focus:ring-2 focus:ring-blue-500
              "
            />
            <span className="text-muted-foreground">—</span>
            <input
              type="number"
              min="1900"
              max="2200"
              value={settings.max_year}
              onChange={(e) => handleChange("max_year", parseInt(e.target.value))}
              className="
                w-24 px-3 py-2 rounded-md
                bg-muted text-foreground
                border border-border
                focus:outline-none focus:ring-2 focus:ring-blue-500
              "
            />
          </div>
        </div>
      </section>

      {/* Save Button */}
      <button
        onClick={saveSettings}
        disabled={saving}
        className="
          w-full py-3 rounded-lg
          bg-blue-500 hover:bg-blue-600
          text-white font-medium
          transition-colors
          disabled:opacity-50 disabled:cursor-not-allowed
        "
      >
        {saving ? t("settings.saving") : t("settings.save")}
      </button>

      {/* App Info */}
      <div className="mt-8 text-center text-xs text-muted-foreground">
        <p>Timesdump v0.1.0</p>
        <p className="mt-1">{t("settings.tagline")}</p>
      </div>
    </div>
  );
}

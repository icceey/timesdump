import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useTranslation } from "react-i18next";

interface Settings {
  min_year: number;
  max_year: number;
  display_duration_ms: number;
  time_format: string;
  hud_position: string;
}

const TIME_FORMATS = [
  { value: "%Y-%m-%d %H:%M:%S", label: "YYYY-MM-DD HH:mm:ss" },
  { value: "%Y/%m/%d %H:%M:%S", label: "YYYY/MM/DD HH:mm:ss" },
  { value: "%d-%m-%Y %H:%M:%S", label: "DD-MM-YYYY HH:mm:ss" },
  { value: "%m/%d/%Y %H:%M:%S", label: "MM/DD/YYYY HH:mm:ss" },
  { value: "%Y-%m-%d", label: "YYYY-MM-DD" },
  { value: "%H:%M:%S", label: "HH:mm:ss" },
];

const HUD_POSITIONS = [
  { value: "top_center", labelKey: "settings.hudPositionTopCenter" },
  { value: "top_left", labelKey: "settings.hudPositionTopLeft" },
  { value: "top_right", labelKey: "settings.hudPositionTopRight" },
  { value: "bottom_center", labelKey: "settings.hudPositionBottomCenter" },
  { value: "bottom_left", labelKey: "settings.hudPositionBottomLeft" },
  { value: "bottom_right", labelKey: "settings.hudPositionBottomRight" },
];

export default function SettingsView() {
  const { t } = useTranslation();
  const [settings, setSettings] = useState<Settings>({
    min_year: 1990,
    max_year: 2050,
    display_duration_ms: 5000,
    time_format: "%Y-%m-%d %H:%M:%S",
    hud_position: "top_center",
  });
  const [autostart, setAutostart] = useState(false);
  const [saving, setSaving] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);

  // Load settings function
  const loadSettingsFromStore = useCallback(async () => {
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
  }, []);

  // Set window title based on current language
  useEffect(() => {
    getCurrentWindow().setTitle(t("settings.title"));
  }, [t]);

  // Load settings on mount and when window gains focus
  useEffect(() => {
    loadSettingsFromStore();

    // Listen for window focus to reload settings when window is shown
    const setupFocusListener = async () => {
      const currentWindow = getCurrentWindow();
      const unlisten = await currentWindow.onFocusChanged(({ payload: focused }) => {
        if (focused) {
          loadSettingsFromStore();
        }
      });
      return unlisten;
    };

    const unlistenPromise = setupFocusListener();

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [loadSettingsFromStore]);

  // Save settings
  const saveSettings = async () => {
    setSaving(true);
    setSaveSuccess(false);
    try {
      await invoke("save_settings", {
        minYear: settings.min_year,
        maxYear: settings.max_year,
        displayDurationMs: settings.display_duration_ms,
        timeFormat: settings.time_format,
        hudPosition: settings.hud_position,
      });
      setSaveSuccess(true);
      // Hide success message after 2 seconds
      setTimeout(() => setSaveSuccess(false), 2000);
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
    <div style={{ 
      minHeight: '100%',
      padding: 20,
      background: 'linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%)'
    }}>
      {/* Title */}
      <h1 style={{
        fontSize: 18,
        fontWeight: 600,
        color: '#1e293b',
        marginBottom: 16
      }}>
        {t("settings.title")}
      </h1>

      {/* Main Settings Card */}
      <div style={{ 
        background: 'white',
        borderRadius: 12,
        overflow: 'hidden',
        marginBottom: 16,
        boxShadow: '0 1px 3px rgba(0,0,0,0.08), 0 1px 2px rgba(0,0,0,0.06)'
      }}>
        {/* Autostart */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '14px 16px',
          borderBottom: '1px solid #f1f5f9'
        }}>
          <span style={{ fontSize: 14, color: '#334155' }}>{t("settings.launchAtLogin")}</span>
          <button
            onClick={toggleAutostart}
            style={{
              width: 44,
              height: 26,
              borderRadius: 13,
              border: 'none',
              background: autostart ? '#22c55e' : '#e2e8f0',
              position: 'relative',
              cursor: 'pointer',
              transition: 'background 0.2s'
            }}
          >
            <span
              style={{
                position: 'absolute',
                top: 2,
                left: autostart ? 20 : 2,
                width: 22,
                height: 22,
                borderRadius: 11,
                background: 'white',
                boxShadow: '0 1px 3px rgba(0,0,0,0.2)',
                transition: 'left 0.2s'
              }}
            />
          </button>
        </div>

        {/* Display Duration */}
        <div style={{
          padding: '14px 16px',
          borderBottom: '1px solid #f1f5f9'
        }}>
          <div style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginBottom: 10
          }}>
            <span style={{ fontSize: 14, color: '#334155' }}>{t("settings.displayDuration")}</span>
            <span style={{ fontSize: 14, fontWeight: 500, color: '#3b82f6' }}>
              {(settings.display_duration_ms / 1000).toFixed(1)}s
            </span>
          </div>
          <input
            type="range"
            min="1500"
            max="10000"
            step="500"
            value={settings.display_duration_ms}
            onChange={(e) => handleChange("display_duration_ms", parseInt(e.target.value))}
            style={{
              width: '100%',
              height: 4,
              borderRadius: 2,
              appearance: 'none',
              background: '#e2e8f0',
              cursor: 'pointer',
              accentColor: '#3b82f6'
            }}
          />
        </div>

        {/* Time Format */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '14px 16px',
          borderBottom: '1px solid #f1f5f9'
        }}>
          <span style={{ fontSize: 14, color: '#334155' }}>{t("settings.timeFormat")}</span>
          <select
            value={settings.time_format}
            onChange={(e) => handleChange("time_format", e.target.value)}
            style={{
              fontSize: 14,
              color: '#64748b',
              background: 'transparent',
              border: 'none',
              outline: 'none',
              textAlign: 'right',
              cursor: 'pointer'
            }}
          >
            {TIME_FORMATS.map((format) => (
              <option key={format.value} value={format.value}>
                {format.label}
              </option>
            ))}
          </select>
        </div>

        {/* HUD Position */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '14px 16px',
          borderBottom: '1px solid #f1f5f9'
        }}>
          <span style={{ fontSize: 14, color: '#334155' }}>{t("settings.hudPosition")}</span>
          <select
            value={settings.hud_position}
            onChange={(e) => handleChange("hud_position", e.target.value)}
            style={{
              fontSize: 14,
              color: '#64748b',
              background: 'transparent',
              border: 'none',
              outline: 'none',
              textAlign: 'right',
              cursor: 'pointer'
            }}
          >
            {HUD_POSITIONS.map((pos) => (
              <option key={pos.value} value={pos.value}>
                {t(pos.labelKey)}
              </option>
            ))}
          </select>
        </div>

        {/* Year Range - inline */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '14px 16px'
        }}>
          <span style={{ fontSize: 14, color: '#334155' }}>{t("settings.yearRange")}</span>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <input
              type="number"
              min="1900"
              max="2200"
              value={settings.min_year}
              onChange={(e) => handleChange("min_year", parseInt(e.target.value))}
              style={{
                width: 70,
                padding: '6px 8px',
                borderRadius: 6,
                border: '1px solid #e2e8f0',
                background: '#f8fafc',
                fontSize: 13,
                fontWeight: 500,
                textAlign: 'center',
                outline: 'none',
                color: '#334155'
              }}
            />
            <span style={{ color: '#94a3b8', fontSize: 12 }}>—</span>
            <input
              type="number"
              min="1900"
              max="2200"
              value={settings.max_year}
              onChange={(e) => handleChange("max_year", parseInt(e.target.value))}
              style={{
                width: 70,
                padding: '6px 8px',
                borderRadius: 6,
                border: '1px solid #e2e8f0',
                background: '#f8fafc',
                fontSize: 13,
                fontWeight: 500,
                textAlign: 'center',
                outline: 'none',
                color: '#334155'
              }}
            />
          </div>
        </div>
      </div>

      {/* Save Button */}
      <button
        onClick={saveSettings}
        disabled={saving}
        style={{
          width: '100%',
          padding: '12px 0',
          borderRadius: 10,
          border: 'none',
          background: saveSuccess ? '#22c55e' : '#3b82f6',
          color: 'white',
          fontSize: 14,
          fontWeight: 600,
          cursor: saving ? 'not-allowed' : 'pointer',
          opacity: saving ? 0.6 : 1,
          transition: 'background 0.2s'
        }}
        onMouseOver={(e) => !saving && !saveSuccess && (e.currentTarget.style.background = '#2563eb')}
        onMouseOut={(e) => !saving && !saveSuccess && (e.currentTarget.style.background = '#3b82f6')}
      >
        {saving ? t("settings.saving") : saveSuccess ? t("settings.saved") : t("settings.save")}
      </button>

      {/* Footer */}
      <p style={{
        textAlign: 'center',
        fontSize: 12,
        color: '#94a3b8',
        marginTop: 16
      }}>
        Timesdump v0.1.0 · {t("settings.tagline")}
      </p>
    </div>
  );
}

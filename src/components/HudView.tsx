import { useEffect, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "react-i18next";

/** Default display duration in milliseconds */
const DEFAULT_DISPLAY_DURATION_MS = 3000;

/** Shorter duration when resuming from hover */
const HOVER_RESUME_DURATION_MS = 2000;

interface HudPayload {
  formatted_time: string;
  raw_value: string;
  timestamp_seconds: number;
  is_milliseconds: boolean;
}

export default function HudView() {
  const { t } = useTranslation();
  const [payload, setPayload] = useState<HudPayload | null>(null);
  const [visible, setVisible] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const [hideTimeout, setHideTimeout] = useState<ReturnType<typeof setTimeout> | null>(null);
  const [displayDuration, setDisplayDuration] = useState(DEFAULT_DISPLAY_DURATION_MS);

  // Load display duration from settings
  useEffect(() => {
    invoke<{ display_duration_ms: number }>("load_settings")
      .then((settings) => {
        if (settings?.display_duration_ms) {
          setDisplayDuration(settings.display_duration_ms);
        }
      })
      .catch(() => {
        // Use default if settings can't be loaded
      });
  }, []);

  // Auto-hide logic
  const scheduleHide = useCallback((duration: number = displayDuration) => {
    if (hideTimeout) {
      clearTimeout(hideTimeout);
    }
    const timeout = setTimeout(() => {
      setVisible(false);
      invoke("hide_hud").catch(console.error);
    }, duration);
    setHideTimeout(timeout);
  }, [hideTimeout, displayDuration]);

  // Listen for show_hud events from Rust
  useEffect(() => {
    const unlisten = listen<HudPayload>("show_hud", (event) => {
      setPayload(event.payload);
      setVisible(true);
      scheduleHide(displayDuration);
    });

    return () => {
      unlisten.then((fn) => fn());
      if (hideTimeout) {
        clearTimeout(hideTimeout);
      }
    };
  }, [scheduleHide, hideTimeout]);

  // Pause hide timer on hover
  useEffect(() => {
    if (isHovered && hideTimeout) {
      clearTimeout(hideTimeout);
      setHideTimeout(null);
    } else if (!isHovered && visible) {
      scheduleHide(HOVER_RESUME_DURATION_MS); // Resume with shorter duration
    }
  }, [isHovered, visible]);

  // Handle click to copy
  const handleClick = async () => {
    if (payload) {
      try {
        await invoke("copy_result", { text: payload.formatted_time });
        setVisible(false);
        await invoke("hide_hud");
      } catch (error) {
        console.error("Failed to copy:", error);
      }
    }
  };

  if (!visible || !payload) {
    return <div className="hud-container h-full" />;
  }

  return (
    <div
      className="hud-container h-full flex items-center justify-center p-4"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div
        onClick={handleClick}
        className="
          no-select cursor-pointer
          bg-white/80 dark:bg-gray-900/80
          backdrop-blur-xl
          rounded-lg
          border border-white/20 dark:border-gray-700/50
          shadow-lg shadow-black/5 dark:shadow-black/20
          px-6 py-4
          min-w-[320px]
          transition-all
          hover:scale-[1.02]
          active:scale-[0.98]
        "
      >
        {/* Main time display */}
        <div className="text-2xl font-mono font-semibold text-gray-900 dark:text-gray-100 text-center">
          {payload.formatted_time}
        </div>
        
        {/* Metadata */}
        <div className="mt-2 flex justify-between items-center text-xs text-gray-500 dark:text-gray-400">
          <span>
            {payload.is_milliseconds ? t("hud.milliseconds") : t("hud.seconds")}
          </span>
          <span className="font-mono opacity-60">
            {payload.raw_value.length > 16 
              ? payload.raw_value.slice(0, 16) + "..." 
              : payload.raw_value}
          </span>
        </div>
        
        {/* Click hint */}
        <div className="mt-3 text-xs text-center text-gray-400 dark:text-gray-500">
          {t("hud.clickToCopy")}
        </div>
      </div>
    </div>
  );
}

import { useEffect, useState, useRef, useCallback } from "react";
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
  relative_time: string;
}

export default function HudView() {
  const { t } = useTranslation();
  const [payload, setPayload] = useState<HudPayload | null>(null);
  const [visible, setVisible] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const [displayDuration, setDisplayDuration] = useState(DEFAULT_DISPLAY_DURATION_MS);
  
  // Use refs to avoid stale closures and prevent effect re-runs
  const hideTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const isHoveredRef = useRef(false);
  const displayDurationRef = useRef(displayDuration);

  // Keep refs in sync
  useEffect(() => {
    isHoveredRef.current = isHovered;
  }, [isHovered]);

  useEffect(() => {
    displayDurationRef.current = displayDuration;
  }, [displayDuration]);

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

  // Clear timeout helper
  const clearHideTimeout = useCallback(() => {
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
      hideTimeoutRef.current = null;
    }
  }, []);

  // Schedule hide with specified duration
  const scheduleHide = useCallback((duration: number) => {
    clearHideTimeout();
    hideTimeoutRef.current = setTimeout(() => {
      setVisible(false);
      invoke("hide_hud").catch(console.error);
    }, duration);
  }, [clearHideTimeout]);

  // Listen for show_hud events from Rust - stable effect, runs once
  useEffect(() => {
    const unlisten = listen<HudPayload>("show_hud", (event) => {
      setPayload(event.payload);
      setVisible(true);
      // Use ref to get current duration value
      scheduleHide(displayDurationRef.current);
    });

    return () => {
      unlisten.then((fn) => fn());
      clearHideTimeout();
    };
  }, [scheduleHide, clearHideTimeout]);

  // Handle hover state changes
  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
    clearHideTimeout();
  }, [clearHideTimeout]);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
    // Only schedule hide if still visible
    if (visible) {
      scheduleHide(HOVER_RESUME_DURATION_MS);
    }
  }, [visible, scheduleHide]);

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
      className="hud-container h-full flex items-center justify-center"
      data-tauri-drag-region
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
    >
      <div
        className="
          no-select cursor-move
          w-full h-full
          flex flex-col items-center justify-center
          px-5 py-3
          transition-opacity
          hover:opacity-90
          active:opacity-75
        "
        data-tauri-drag-region
      >
        {/* Main time display */}
        <div className="text-[22px] font-medium tracking-tight text-black/85 dark:text-white/90 text-center font-mono">
          {payload.formatted_time}
        </div>
        
        {/* Relative time display */}
        <div className="mt-1 text-[13px] text-black/60 dark:text-white/65 tracking-wide">
          {payload.relative_time}
        </div>
        
        {/* Metadata row */}
        <div className="mt-1.5 flex items-center gap-3 text-[11px] text-black/45 dark:text-white/50">
          <span className="uppercase tracking-wide">
            {payload.is_milliseconds ? t("hud.milliseconds") : t("hud.seconds")}
          </span>
          <span className="text-black/25 dark:text-white/25">•</span>
          <span className="font-mono">
            {payload.raw_value.length > 13 
              ? payload.raw_value.slice(0, 13) 
              : payload.raw_value}
          </span>
        </div>
        
        {/* Click hint */}
        <div className="mt-2 text-[10px] text-black/35 dark:text-white/40 tracking-wide">
          {t("hud.clickToCopy")}
        </div>
      </div>
    </div>
  );
}

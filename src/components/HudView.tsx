import { useEffect, useState, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "react-i18next";

/** Default display duration in milliseconds */
const DEFAULT_DISPLAY_DURATION_MS = 3000;

/** Shorter duration when resuming from hover */
const HOVER_RESUME_DURATION_MS = 2000;

/** Constants for relative time calculation */
const DAYS_PER_MONTH = 30;
const DAYS_PER_YEAR = 365;

interface HudPayload {
  formatted_time: string;
  raw_value: string;
  timestamp_seconds: number;
  is_milliseconds: boolean;
}

/** Calculate relative time from timestamp */
function calculateRelativeTime(timestampSeconds: number, t: (key: string, options?: any) => string): string {
  const now = Math.floor(Date.now() / 1000);
  const diffSeconds = timestampSeconds - now;
  const absDiff = Math.abs(diffSeconds);
  const isPast = diffSeconds < 0;

  // Calculate time units
  const seconds = Math.floor(absDiff);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const months = Math.floor(days / DAYS_PER_MONTH);
  const years = Math.floor(days / DAYS_PER_YEAR);

  // Format the relative time string with i18n
  if (years > 0) {
    return t(isPast ? "hud.relative.yearsAgo" : "hud.relative.yearsLater", { count: years });
  } else if (months > 0) {
    return t(isPast ? "hud.relative.monthsAgo" : "hud.relative.monthsLater", { count: months });
  } else if (days > 0) {
    return t(isPast ? "hud.relative.daysAgo" : "hud.relative.daysLater", { count: days });
  } else if (hours > 0) {
    return t(isPast ? "hud.relative.hoursAgo" : "hud.relative.hoursLater", { count: hours });
  } else if (minutes > 0) {
    return t(isPast ? "hud.relative.minutesAgo" : "hud.relative.minutesLater", { count: minutes });
  } else {
    return t(isPast ? "hud.relative.secondsAgo" : "hud.relative.secondsLater", { count: seconds });
  }
}

export default function HudView() {
  const { t } = useTranslation();
  const [payload, setPayload] = useState<HudPayload | null>(null);
  const [visible, setVisible] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
  const [copySuccess, setCopySuccess] = useState(false);
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
      setIsPinned(false); // Reset pin state on new timestamp
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
    // Only schedule hide if still visible and not pinned
    if (visible && !isPinned) {
      scheduleHide(HOVER_RESUME_DURATION_MS);
    }
  }, [visible, isPinned, scheduleHide]);

  // Handle copy action
  const handleCopy = useCallback(async () => {
    if (payload && !copySuccess) {
      try {
        await invoke("copy_result", { text: payload.formatted_time });
        setCopySuccess(true);
        setTimeout(() => setCopySuccess(false), 1500);
      } catch (error) {
        console.error("Failed to copy:", error);
      }
    }
  }, [payload, copySuccess]);

  // Handle pin toggle
  const handlePinToggle = useCallback(() => {
    setIsPinned((prev) => {
      const newPinned = !prev;
      if (newPinned) {
        // When pinning, clear any pending hide timeout
        clearHideTimeout();
      } else {
        // When unpinning, schedule hide
        scheduleHide(HOVER_RESUME_DURATION_MS);
      }
      return newPinned;
    });
  }, [clearHideTimeout, scheduleHide]);

  // Handle close action
  const handleClose = useCallback(async () => {
    setVisible(false);
    setIsPinned(false);
    await invoke("hide_hud").catch(console.error);
  }, []);

  if (!visible || !payload) {
    return <div className="hud-container h-full" />;
  }

  return (
    <div
      className="hud-container h-full flex items-center"
      style={{ paddingRight: "12px" }}
      data-tauri-drag-region
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Content area */}
      <div
        className="
          no-select cursor-move pointer-events-none
          flex-1 h-full
          flex flex-col items-center justify-center
          pl-5 pr-3 py-3
        "
      >
        {/* Main time display */}
        <div className="text-[22px] font-medium tracking-tight text-black/85 dark:text-white/90 text-center font-mono">
          {payload.formatted_time}
        </div>
        
        {/* Relative time display */}
        <div className="mt-1 text-[13px] text-black/60 dark:text-white/65 tracking-wide">
          {calculateRelativeTime(payload.timestamp_seconds, t)}
        </div>
        
        {/* Metadata row */}
        <div className="mt-1.5 text-[13px] text-black/45 dark:text-white/50">
          <span className="font-mono">
            {payload.raw_value.length > 13 
              ? payload.raw_value.slice(0, 13) 
              : payload.raw_value}
          </span>
        </div>
      </div>

      {/* Action buttons */}
      <div className="flex flex-col gap-1.5 p-3">
        {/* Copy button */}
        <button
          onClick={handleCopy}
          className={`
            w-7 h-7 flex items-center justify-center
            rounded-md
            transition-colors
            ${copySuccess
              ? "bg-green-500/80 text-white"
              : "bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/20 active:bg-black/15 dark:active:bg-white/25 text-black/60 dark:text-white/70"
            }
          `}
          title={t("hud.copy")}
        >
          {copySuccess ? (
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          ) : (
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
              <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
            </svg>
          )}
        </button>

        {/* Pin button */}
        <button
          onClick={handlePinToggle}
          className={`
            w-7 h-7 flex items-center justify-center
            rounded-md
            transition-colors
            ${isPinned 
              ? "bg-blue-500/80 text-white hover:bg-blue-500/90 active:bg-blue-600"
              : "bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/20 active:bg-black/15 dark:active:bg-white/25 text-black/60 dark:text-white/70"
            }
          `}
          title={t(isPinned ? "hud.unpin" : "hud.pin")}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <line x1="12" x2="12" y1="17" y2="22"/>
            <path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"/>
          </svg>
        </button>

        {/* Close button */}
        <button
          onClick={handleClose}
          className="
            w-7 h-7 flex items-center justify-center
            rounded-md
            bg-black/5 dark:bg-white/10
            hover:bg-red-500/80 hover:text-white
            active:bg-red-600
            text-black/60 dark:text-white/70
            transition-colors
          "
          title={t("hud.close")}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M18 6 6 18"/>
            <path d="m6 6 12 12"/>
          </svg>
        </button>
      </div>
    </div>
  );
}

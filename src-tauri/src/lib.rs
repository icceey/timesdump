use arboard::Clipboard;
use chrono::{Datelike, TimeZone, Utc};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

mod ghost_window;
mod tray;

pub use ghost_window::setup_ghost_window;
pub use tray::setup_tray_menu;

/// HUD popup position on screen
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum HudPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    #[default]
    TopCenter,
    BottomCenter,
}

/// Configuration for timestamp parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampConfig {
    pub min_year: i32,
    pub max_year: i32,
    pub display_duration_ms: u64,
    pub time_format: String,
    #[serde(default)]
    pub hud_position: HudPosition,
}

impl Default for TimestampConfig {
    fn default() -> Self {
        Self {
            min_year: 1990,
            max_year: 2050,
            display_duration_ms: 5000,
            time_format: "%Y-%m-%d %H:%M:%S".to_string(),
            hud_position: HudPosition::default(),
        }
    }
}

/// Payload for the show_hud event
#[derive(Debug, Clone, Serialize)]
pub struct HudPayload {
    pub formatted_time: String,
    pub raw_value: String,
    pub timestamp_seconds: i64,
    pub is_milliseconds: bool,
    pub relative_time: String,
}

/// TimeParser handles validation and parsing of timestamp strings
pub struct TimeParser {
    config: TimestampConfig,
}

impl TimeParser {
    pub fn new(config: TimestampConfig) -> Self {
        Self { config }
    }

    pub fn update_config(&mut self, config: TimestampConfig) {
        self.config = config;
    }

    /// Get the configured HUD position
    pub fn get_hud_position(&self) -> HudPosition {
        self.config.hud_position
    }

    /// Calculate relative time from now
    fn calculate_relative_time(timestamp_seconds: i64) -> String {
        let now = Utc::now().timestamp();
        let diff_seconds = timestamp_seconds - now;
        let abs_diff = diff_seconds.abs();

        // Determine if past or future
        let is_past = diff_seconds < 0;

        // Calculate time units
        let seconds = abs_diff;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        let months = days / 30;
        let years = days / 365;

        // Format the relative time string
        if years > 0 {
            format!(
                "{} year{} {}",
                years,
                if years > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        } else if months > 0 {
            format!(
                "{} month{} {}",
                months,
                if months > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        } else if days > 0 {
            format!(
                "{} day{} {}",
                days,
                if days > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        } else if hours > 0 {
            format!(
                "{} hour{} {}",
                hours,
                if hours > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        } else if minutes > 0 {
            format!(
                "{} minute{} {}",
                minutes,
                if minutes > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        } else {
            format!(
                "{} second{} {}",
                seconds,
                if seconds > 1 { "s" } else { "" },
                if is_past { "ago" } else { "later" }
            )
        }
    }

    /// Parse a clipboard string and return HudPayload if valid
    pub fn parse(&self, input: &str) -> Option<HudPayload> {
        // Step 1: Trim whitespace
        let trimmed = input.trim();

        // Step 2: Check if string is non-empty and all digits
        if trimmed.is_empty() || !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        // Step 3: Parse as number
        let value: i64 = trimmed.parse().ok()?;

        // Step 4: Determine if seconds or milliseconds based on length
        let (timestamp_seconds, is_milliseconds) = if trimmed.len() <= 10 {
            // Seconds
            (value, false)
        } else {
            // Milliseconds - convert to seconds
            (value / 1000, true)
        };

        // Step 5: Convert to DateTime and check year range
        let datetime = Utc.timestamp_opt(timestamp_seconds, 0).single()?;
        let year = datetime.year();

        if year < self.config.min_year || year > self.config.max_year {
            debug!(
                "Year {} out of range [{}, {}]",
                year, self.config.min_year, self.config.max_year
            );
            return None;
        }

        // Step 6: Format the time
        let formatted_time = datetime.format(&self.config.time_format).to_string();

        // Step 7: Calculate relative time
        let relative_time = Self::calculate_relative_time(timestamp_seconds);

        Some(HudPayload {
            formatted_time,
            raw_value: trimmed.to_string(),
            timestamp_seconds,
            is_milliseconds,
            relative_time,
        })
    }
}

/// Default clipboard polling interval in milliseconds
const CLIPBOARD_POLL_INTERVAL_MS: u64 = 350;

/// ClipboardMonitor polls the clipboard and emits events when valid timestamps are detected
pub struct ClipboardMonitor {
    parser: Arc<Mutex<TimeParser>>,
    last_content: Arc<Mutex<String>>,
    running: Arc<Mutex<bool>>,
}

impl ClipboardMonitor {
    pub fn new(config: TimestampConfig) -> Self {
        // Read current clipboard content to avoid triggering on startup
        let initial_content = Clipboard::new()
            .and_then(|mut c| c.get_text())
            .unwrap_or_default();

        Self {
            parser: Arc::new(Mutex::new(TimeParser::new(config))),
            last_content: Arc::new(Mutex::new(initial_content)),
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn update_config(&self, config: TimestampConfig) {
        if let Ok(mut parser) = self.parser.lock() {
            parser.update_config(config);
        }
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    pub fn set_running(&self, running: bool) {
        *self.running.lock().unwrap() = running;
    }

    pub fn toggle_pause(&self) -> bool {
        let mut running = self.running.lock().unwrap();
        *running = !*running;
        *running
    }

    /// Start the clipboard monitoring thread
    pub fn start(&self, app_handle: AppHandle) {
        let parser = Arc::clone(&self.parser);
        let last_content = Arc::clone(&self.last_content);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            let mut clipboard = match Clipboard::new() {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to access clipboard: {}", e);
                    return;
                }
            };

            loop {
                thread::sleep(Duration::from_millis(CLIPBOARD_POLL_INTERVAL_MS));

                // Check if monitoring is paused
                if !*running.lock().unwrap() {
                    continue;
                }

                // Get current clipboard text
                let current = match clipboard.get_text() {
                    Ok(text) => text,
                    Err(_) => continue,
                };

                // Check if content changed
                let mut last = last_content.lock().unwrap();
                if current == *last {
                    continue;
                }
                *last = current.clone();
                drop(last);

                // Try to parse as timestamp
                if let Ok(parser_guard) = parser.lock() {
                    if let Some(payload) = parser_guard.parse(&current) {
                        info!("Valid timestamp detected: {}", payload.formatted_time);

                        // Clone payload and get position for the closure
                        let payload_clone = payload.clone();
                        let hud_position = parser_guard.get_hud_position();
                        let app_handle_clone = app_handle.clone();

                        // Position and show the HUD window on the main thread
                        // macOS requires all UI operations to run on the main thread
                        let _ = app_handle.run_on_main_thread(move || {
                            if let Some(hud_window) = app_handle_clone.get_webview_window("hud") {
                                // Position window at the configured fixed position
                                #[cfg(target_os = "macos")]
                                ghost_window::position_hud_macos(&hud_window, hud_position);

                                #[cfg(target_os = "windows")]
                                ghost_window::position_hud_windows(&hud_window, hud_position);

                                #[cfg(target_os = "linux")]
                                ghost_window::position_hud_linux(&hud_window, hud_position);
                            }

                            // Emit event to frontend
                            if let Err(e) = app_handle_clone.emit("show_hud", payload_clone) {
                                error!("Failed to emit show_hud event: {}", e);
                            }
                        });
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seconds_timestamp() {
        let config = TimestampConfig::default();
        let parser = TimeParser::new(config);

        // 2024-01-01 00:00:00 in seconds
        let result = parser.parse("1704067200");
        assert!(result.is_some());
        let payload = result.unwrap();
        assert_eq!(payload.timestamp_seconds, 1704067200);
        assert!(!payload.is_milliseconds);
    }

    #[test]
    fn test_parse_milliseconds_timestamp() {
        let config = TimestampConfig::default();
        let parser = TimeParser::new(config);

        // 2024-01-01 00:00:00 in milliseconds
        let result = parser.parse("1704067200000");
        assert!(result.is_some());
        let payload = result.unwrap();
        assert_eq!(payload.timestamp_seconds, 1704067200);
        assert!(payload.is_milliseconds);
    }

    #[test]
    fn test_reject_non_numeric() {
        let config = TimestampConfig::default();
        let parser = TimeParser::new(config);

        assert!(parser.parse("hello").is_none());
        assert!(parser.parse("123abc").is_none());
        assert!(parser.parse("12.34").is_none());
    }

    #[test]
    fn test_reject_out_of_range_year() {
        let config = TimestampConfig {
            min_year: 2000,
            max_year: 2030,
            ..Default::default()
        };
        let parser = TimeParser::new(config);

        // 1990 is out of range
        assert!(parser.parse("631152000").is_none());
    }

    #[test]
    fn test_trim_whitespace() {
        let config = TimestampConfig::default();
        let parser = TimeParser::new(config);

        let result = parser.parse("  1704067200  ");
        assert!(result.is_some());
    }

    #[test]
    fn test_relative_time_included() {
        let config = TimestampConfig::default();
        let parser = TimeParser::new(config);

        // Parse a timestamp from the past (2024-01-01)
        let result = parser.parse("1704067200");
        assert!(result.is_some());
        let payload = result.unwrap();
        
        // Verify relative_time field is populated and contains expected keywords
        assert!(!payload.relative_time.is_empty());
        assert!(
            payload.relative_time.contains("ago") || payload.relative_time.contains("later"),
            "relative_time should contain 'ago' or 'later', got: {}",
            payload.relative_time
        );
    }
}

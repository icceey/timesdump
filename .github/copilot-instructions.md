# Copilot Instructions for Timesdump

> **Trust these instructions.** Only search the codebase if information here is incomplete or incorrect.

## Critical Rules

1. **Always run CI check before committing** - Run `npm run check` after ANY code change. Use `npm run check:fix` to auto-fix Rust formatting first. **NEVER commit code that fails CI checks.**
2. **Consult documentation when uncertain** - Check [docs.rs](https://docs.rs), [Tauri v2 docs](https://v2.tauri.app/reference/config/), or relevant API references.
3. **Update README if needed** - When adding features or changing commands.

---

## Project Summary

**Timesdump** is a Tauri v2 cross-platform desktop app that monitors the clipboard for Unix timestamps and displays them as human-readable dates in a non-focus-stealing HUD popup. Runs silently in system tray.

| Component | Technology |
|-----------|------------|
| Backend | Rust (2021 edition) |
| Frontend | React 19 + TypeScript + Vite 7 |
| Framework | Tauri v2 (with autostart, positioner, store plugins) |
| Styling | Tailwind CSS v4 |
| i18n | react-i18next |

---

## Commands

```bash
npm run check        # CI validation (TypeScript, Rust format, Clippy, tests)
npm run check:fix    # Auto-fix Rust formatting, then validate
npm run tauri dev    # Hot reload dev server (port 1420)
npm run tauri build  # Production build
```

---

## Key Architecture

1. **HUD window MUST never steal focus** - `ghost_window.rs` uses platform-specific non-activating window APIs
2. **Two windows:** `hud` (transparent overlay) and `settings` (standard window)
3. **Clipboard monitoring:** `ClipboardMonitor` in `lib.rs` polls every 350ms via `arboard` crate
4. **Year-range filtering:** Excludes phone numbers/verification codes (configurable min/max year)

---

## Core Structs (lib.rs)

```rust
// Configuration - stored in settings.json via tauri-plugin-store
pub struct TimestampConfig {
    pub min_year: i32,              // Filter: minimum valid year
    pub max_year: i32,              // Filter: maximum valid year
    pub display_duration_ms: u64,   // HUD display time in ms
    pub time_format: String,        // chrono format string
    pub hud_position: HudPosition,  // TopLeft, TopRight, BottomLeft, BottomRight, TopCenter, BottomCenter
}

// Event payload sent to HUD via app_handle.emit("show_hud", payload)
pub struct HudPayload {
    pub formatted_time: String,     // Formatted datetime string
    pub raw_value: String,          // Original clipboard text
    pub timestamp_seconds: i64,     // Parsed Unix timestamp
    pub is_milliseconds: bool,      // Was input 13-digit ms?
}
```

---

## Key Files

| Task | File(s) |
|------|---------|
| Add Tauri command | `main.rs` → `invoke_handler![]` |
| Core structs/logic | `lib.rs` → `TimestampConfig`, `HudPayload`, `TimeParser`, `ClipboardMonitor` |
| Platform window APIs | `lib.rs` → `ghost_window` module |
| Tray menu | `lib.rs` → `tray` module |
| Window definitions | `tauri.conf.json` → `app.windows` |
| Add UI setting | `SettingsView.tsx` |
| Add translation | `src/locales/en.json` + `zh-CN.json` (BOTH required) |

---

## Common Patterns

### Adding a New Setting
1. Add field to `TimestampConfig` in `lib.rs` with `#[serde(default)]` if optional
2. Update `load_settings`/`save_settings` in `main.rs` (individual params, not struct)
3. Add UI in `SettingsView.tsx`
4. Add i18n keys to BOTH locale files

### Rust ↔ Frontend Communication
```typescript
// Command (request-response)
const result = await invoke("command_name", { param1: value });

// Event (push from Rust)
const unlisten = await listen("show_hud", (event) => { /* handle payload */ });
```

### Platform-Specific Code
```rust
#[cfg(target_os = "macos")]
use objc2_app_kit::NSWindow;  // Uses objc2 bindings

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;
```

---

## Tauri Commands (main.rs)

| Command | Signature |
|---------|-----------|
| `get_system_locale` | `() -> String` |
| `copy_result` | `(text: String) -> Result<(), String>` |
| `hide_hud` | `async (app: AppHandle) -> Result<(), String>` |
| `show_settings` | `async (app: AppHandle) -> Result<(), String>` |
| `toggle_pause` | `(state: State<Arc<ClipboardMonitor>>) -> bool` |
| `save_settings` | `async (app, min_year, max_year, display_duration_ms, time_format, hud_position)` |
| `load_settings` | `async (app: AppHandle) -> Result<TimestampConfig, String>` |

---

## Testing

- **Rust tests:** `cargo test --lib` (TimeParser validation)
- **TypeScript:** `npx tsc --noEmit`
- **Test timestamps:** `1704067200` (10-digit seconds), `1704067200000` (13-digit ms)

---

## Known Issues

1. **Rust formatting:** CI enforces `cargo fmt`. Run `npm run check:fix` before committing.
2. **Port 1420 conflict:** Kill existing processes if dev server fails to start.
3. **macOS:** Requires Xcode CLI tools (`xcode-select --install`)
4. **Windows:** Requires VS Build Tools with C++ workload

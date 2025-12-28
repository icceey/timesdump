# Copilot Instructions for Timesdump

> **Trust these instructions.** Only search the codebase if information here is incomplete or incorrect.

## Critical Rules

1. **Always run CI check before committing** - After ANY code change, run `npm run check` to validate all CI checks pass. Use `npm run check:fix` to auto-fix Rust formatting issues first. **NEVER commit code that fails CI checks.**
2. **Consult documentation when uncertain** - Check local docs, [docs.rs](https://docs.rs), [Tauri v2 docs](https://v2.tauri.app/reference/config/), or relevant API references before implementing unfamiliar features.
3. **Update README if needed** - When adding features, changing commands, or modifying dependencies, check if `README.md` needs corresponding updates.

---

## Project Summary

**Timesdump** is a Tauri v2 cross-platform desktop app (~2,500 lines) that monitors the clipboard for Unix timestamps and displays them as human-readable dates in a non-focus-stealing HUD popup. Runs silently in system tray with no dock/taskbar icon.

| Component | Technology |
|-----------|------------|
| Backend | Rust (2021 edition) |
| Frontend | React 19 + TypeScript + Vite 7 |
| Framework | Tauri v2 |
| Styling | Tailwind CSS v4 |
| i18n | react-i18next |

---

## Build & Validation Commands

**ALWAYS run CI check after finishing changes or before committing. CI runs all checks on Linux.**

### Quick CI Validation (REQUIRED before committing)
```bash
npm run check        # Run all CI checks (TypeScript, Rust format, Clippy, tests)
npm run check:fix    # Auto-fix Rust formatting first, then run all checks
```

### Individual Commands (for reference)
```bash
npm install                         # Install dependencies (run first)
npx tsc --noEmit                    # TypeScript type check
cd src-tauri && cargo fmt --all -- --check   # Rust format check
cd src-tauri && cargo clippy --all-targets -- -D warnings  # Clippy lint
cd src-tauri && cargo test --lib    # Unit tests
```

### Development & Build
```bash
npm run tauri dev                   # Hot reload dev server on port 1420
npm run tauri build                 # Production build
```

### Fixing Format Issues
```bash
cd src-tauri && cargo fmt --all     # Auto-fix Rust formatting
# Or use: npm run check:fix         # Fix and validate in one command
```

---

## CI Pipeline (GitHub Actions)

The `build.yml` workflow runs on every PR to `master`:

| Step | Platform | Command |
|------|----------|---------|
| TypeScript check | Linux | `npx tsc --noEmit` |
| Rust format | Linux | `cargo fmt --all -- --check` |
| Clippy lint | Linux | `cargo clippy --all-targets -- -D warnings` |
| Unit tests | Linux | `cargo test --lib` |
| App build | macOS, Windows | `npm run tauri build` |

**Key:** Linux runs ALL lint/test checks. macOS/Windows only run builds.

---

## Project Layout

```
timesdump/
├── src/                          # Frontend (React + TypeScript)
│   ├── App.tsx                   # View router (hash-based: #/hud, #/settings)
│   ├── components/
│   │   ├── HudView.tsx           # HUD popup display
│   │   └── SettingsView.tsx      # Settings panel UI
│   ├── lib/i18n.ts               # Internationalization setup
│   └── locales/
│       ├── en.json               # English strings
│       └── zh-CN.json            # Chinese strings
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── main.rs               # Entry point, Tauri commands, app setup
│   │   ├── lib.rs                # ClipboardMonitor, TimeParser, structs
│   │   ├── ghost_window.rs       # Platform-specific window APIs
│   │   └── tray.rs               # System tray menu
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Window defs, plugins, bundle config
├── package.json                  # Node.js deps (requires Node 24+)
├── vite.config.ts                # Vite config (port 1420)
└── tsconfig.json                 # TypeScript strict mode config
```

---

## Key Architecture Rules

1. **HUD window MUST never steal focus** - uses platform-specific non-activating window APIs
2. **Two windows:** `hud` (transparent overlay) and `settings` (standard window)
3. **Clipboard polling:** 350ms intervals in dedicated Rust thread
4. **Year-range filtering:** Excludes phone numbers/verification codes

---

## Key Files by Purpose

| Task | File(s) |
|------|---------|
| Add Tauri command | `src-tauri/src/main.rs` (invoke_handler) |
| Core logic/structs | `src-tauri/src/lib.rs` (TimestampConfig, HudPayload) |
| Platform window behavior | `src-tauri/src/ghost_window.rs` |
| Tray menu items | `src-tauri/src/tray.rs` |
| Window definitions | `src-tauri/tauri.conf.json` → app.windows |
| Add UI setting | `src/components/SettingsView.tsx` |
| Add translation | `src/locales/en.json` + `src/locales/zh-CN.json` |

---

## Common Patterns

### Adding a New Setting
1. Add field to `TimestampConfig` struct in `lib.rs`
2. Update `load_settings`/`save_settings` commands in `main.rs`
3. Add UI control in `SettingsView.tsx`
4. Add translation keys to BOTH `src/locales/en.json` and `src/locales/zh-CN.json`

### Rust → Frontend Communication
- **Command:** `invoke("command_name", { args })` from frontend
- **Event:** `app_handle.emit("event_name", payload)` → `listen("event_name", callback)`

### Platform-Specific Rust Code
```rust
#[cfg(target_os = "macos")]
fn setup_macos() { /* ... */ }

#[cfg(target_os = "windows")]
fn setup_windows() { /* ... */ }

#[cfg(target_os = "linux")]
fn setup_linux() { /* ... */ }
```

---

## Tauri Commands Reference

Defined in `main.rs`, callable via `invoke()`:

| Command | Purpose |
|---------|---------|
| `get_system_locale` | Get OS locale for i18n |
| `copy_result` | Copy string to clipboard |
| `hide_hud` | Hide the HUD window |
| `show_settings` | Show settings window |
| `toggle_pause` | Pause/resume clipboard monitoring |
| `save_settings` | Persist settings to store |
| `load_settings` | Load settings from store |

---

## Testing

- **Rust unit tests:** `cargo test --lib` (5 tests for TimeParser)
- **TypeScript check:** `npx tsc --noEmit`
- **Manual testing required** for platform-specific window behavior
- Test timestamps: seconds (10 digits, e.g. `1704067200`), milliseconds (13 digits, e.g. `1704067200000`)

---

## Dependencies & Versions

| Tool | Version | Notes |
|------|---------|-------|
| Node.js | ≥24.0.0 | Required in package.json engines |
| Rust | stable (2021 edition) | Latest stable recommended |
| Tauri CLI | v2.x | Installed via npm devDependencies |

### Linux System Dependencies (for CI)
```bash
sudo apt-get install -y pkg-config build-essential libglib2.0-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev
```

---

## Known Issues & Workarounds

1. **Rust formatting:** CI enforces `cargo fmt --all -- --check`. Always run `cargo fmt --all` before committing.
2. **Port conflict:** Dev server uses port 1420. Kill existing processes if blocked.
3. **macOS Xcode:** Requires Xcode Command Line Tools (`xcode-select --install`)
4. **Windows build:** Requires Visual Studio Build Tools with C++ workload

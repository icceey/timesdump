# Copilot Instructions for Timesdump

## Project Overview

Timesdump is a **Tauri v2** cross-platform desktop app that monitors the clipboard for Unix timestamps and displays them as human-readable dates in a non-focus-stealing HUD popup. The app runs silently in the system tray with no dock/taskbar icon.

## Architecture

```
Frontend (React + TypeScript)     ←── Tauri IPC ──→     Backend (Rust)
src/                                                     src-tauri/src/
├── components/HudView.tsx         invoke/emit            ├── lib.rs (ClipboardMonitor, TimeParser)
├── components/SettingsView.tsx                           ├── ghost_window.rs (platform window APIs)
└── lib/i18n.ts                                           └── tray.rs (system tray menu)
```

**Key design constraints:**
- HUD window **must never steal focus** - uses platform-specific non-activating window APIs
- Two windows: `hud` (transparent overlay) and `settings` (standard window)
- Clipboard polling at 350ms intervals in a dedicated Rust thread
- Year-range filtering to exclude phone numbers/verification codes

## Development Commands

```bash
npm install              # Install frontend dependencies
npm run tauri dev        # Run in development mode (hot reload)
npm run tauri build      # Build for production

# Rust-specific
cd src-tauri
cargo fmt --all          # Format Rust code
cargo clippy --all-targets -- -D warnings   # Lint Rust code
cargo test --lib         # Run Rust unit tests
```

## Code Quality Requirements

**Before committing code, ALL of the following must pass:**
```bash
# Frontend
npx tsc --noEmit                             # TypeScript type check

# Backend
cd src-tauri
cargo fmt --all --check                      # Check Rust formatting
cargo clippy --all-targets -- -D warnings    # Rust linting (no warnings allowed)
cargo test --lib                             # Run all unit tests
```

**Before implementing features:**
1. **Read existing code thoroughly** - Ensure full understanding of current logic before making changes
2. **Consult documentation when uncertain** - Check local docs, [docs.rs](https://docs.rs), Tauri docs, or relevant API references
3. **Understand cross-component impact** - Changes in `lib.rs` may affect frontend event handling; changes in `ghost_window.rs` require platform-specific testing

## Key Patterns

### Rust Backend

- **Tauri commands** in `lib.rs`: `load_settings`, `save_settings`, `copy_result`, `hide_hud`
- **Event emission**: `app_handle.emit("show_hud", HudPayload)` to notify frontend
- **Platform-specific code**: Use `#[cfg(target_os = "...")]` attributes in `ghost_window.rs`
- **Settings storage**: Uses `tauri-plugin-store` for persistent JSON config

### Frontend

- **View routing**: URL hash-based (`#/hud` or `#/settings`) in `App.tsx`
- **Tauri API**: `invoke()` for commands, `listen()` for events from `@tauri-apps/api`
- **i18n**: Uses `react-i18next` with locale files in `src/locales/{en,zh-CN}.json`
- **Styling**: Tailwind CSS v4

### Adding New Settings

1. Add field to `TimestampConfig` struct in `lib.rs`
2. Update `load_settings`/`save_settings` commands
3. Add UI control in `SettingsView.tsx`
4. Add translation keys to `src/locales/*.json`

### Platform Window Handling

The `ghost_window.rs` file contains critical platform-specific code:
- **macOS**: Uses `NSPanel` with `NS_NONACTIVATING_PANEL_MASK`, vibrancy effects
- **Windows**: Uses `WS_EX_NOACTIVATE` via Win32 API, Mica/Acrylic backdrop
- **Linux**: Basic GTK hints (limited vibrancy support)

## Important Files

| File | Purpose |
|------|---------|
| [src-tauri/src/lib.rs](src-tauri/src/lib.rs) | Core logic: clipboard monitor, timestamp parser, Tauri commands |
| [src-tauri/src/ghost_window.rs](src-tauri/src/ghost_window.rs) | Platform-specific non-activating window setup |
| [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) | Window definitions, plugins, bundle config |
| [src/components/HudView.tsx](src/components/HudView.tsx) | HUD display with auto-hide and hover pause |
| [src/lib/i18n.ts](src/lib/i18n.ts) | Internationalization setup |

## Testing Considerations

- Rust unit tests exist for `TimeParser` in `lib.rs` - run with `cargo test --lib`
- Frontend has TypeScript checking: `npx tsc --noEmit`
- Manual testing required for platform-specific window behavior
- Test with various timestamp formats: seconds (10 digits), milliseconds (13 digits)

## Localization

When adding user-facing strings:
1. Add keys to both `src/locales/en.json` and `src/locales/zh-CN.json`
2. Use `useTranslation()` hook and `t('key.path')` in React components
3. Language is auto-detected from system locale via `get_system_locale` Rust command

# Timesdump

> The Silent Timestamp Decoder

[![Build](https://github.com/icceey/timesdump/actions/workflows/build.yml/badge.svg)](https://github.com/icceey/timesdump/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Timesdump is a cross-platform desktop application that silently monitors your clipboard for Unix timestamps and displays them as human-readable dates without stealing focus from your current work.

## Features

- **Silent Operation**: Runs in the background with no dock/taskbar icon
- **Non-Focus Stealing**: HUD popup never interrupts your typing flow
- **Smart Detection**: Automatically distinguishes between second and millisecond timestamps
- **Year Range Filter**: Filters out phone numbers and verification codes
- **Native Experience**: Uses platform-native blur effects (Mica/Acrylic on Windows, Vibrancy on macOS)
- **Localization**: Supports English and Simplified Chinese

## Tech Stack

- **Framework**: Tauri v2
- **Backend**: Rust
- **Frontend**: React + TypeScript + Vite
- **Styling**: Tailwind CSS

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools with C++ workload
  - **Linux**: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

### Setup

1. Install dependencies:

```bash
npm install
```

2. Run in development mode:

```bash
npm run tauri dev
```

3. Build for production:

```bash
npm run tauri build
```

## Usage

1. Launch Timesdump - it will appear as an icon in your system tray (menu bar on macOS)
2. Copy any Unix timestamp to your clipboard (e.g., `1704067200` or `1704067200000`)
3. A floating HUD will appear near your cursor showing the formatted date/time
4. Click the HUD to copy the formatted time, or wait for it to auto-dismiss

### System Tray Menu

- **Left Click**: Open Settings
- **Right Click**: Show context menu with Pause/Resume, Settings, and Quit options

### Settings

- **Launch at Login**: Start Timesdump automatically when you log in
- **Display Duration**: How long the HUD stays visible (1.5s - 10s)
- **Time Format**: Choose your preferred date/time format
- **Year Range**: Filter timestamps to a specific year range

## Architecture

```
timesdump/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utilities and i18n
│   └── locales/            # Translation files
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── lib.rs          # Core logic and commands
│   │   ├── ghost_window.rs # Platform-specific window handling
│   │   └── tray.rs         # System tray implementation
│   └── Cargo.toml          # Rust dependencies
├── package.json            # Node.js dependencies
└── tauri.conf.json         # Tauri configuration
```

## License

MIT

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

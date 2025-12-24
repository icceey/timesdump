# Timesdump

**The Silent Timestamp Decoder**

<p align="center">
  <img src="https://img.shields.io/badge/macOS-14.0%2B-blue?style=flat-square&logo=apple" alt="macOS 14.0+">
  <img src="https://img.shields.io/badge/Windows-10%2B-0078d4?style=flat-square&logo=windows" alt="Windows 10+">
  <img src="https://img.shields.io/badge/Swift-6-orange?style=flat-square&logo=swift" alt="Swift 6">
  <img src="https://img.shields.io/badge/.NET-10-512bd4?style=flat-square&logo=dotnet" alt=".NET 10">
</p>

## Overview

Timesdump is a minimalist, cross-platform utility that silently monitors your clipboard for Unix timestamps and displays them as human-readable dates in a beautiful, non-intrusive HUD overlay.

### Key Features

- **Silent Operation**: Runs in the background with no dock icon (macOS) or taskbar presence (Windows)
- **Non-Activating HUD**: The overlay window never steals focus from your active application
- **Smart Parsing**: Automatically detects and converts both second and millisecond timestamps
- **Range Filtering**: Filters out phone numbers, verification codes, and other non-timestamp numeric data
- **Native Design**: Uses platform-native UI frameworks for the best user experience
- **Bilingual**: Supports English and Simplified Chinese

## Screenshots

### macOS
- Spotlight-style HUD with ultra-thin material backdrop
- Menu bar integration with quick access to settings

### Windows
- Fluent Design HUD with Mica/Acrylic backdrop
- System tray integration with context menu

## Installation

### macOS

1. Download the latest `.app` from the [Releases](../../releases) page
2. Move `Timesdump.app` to your Applications folder
3. Launch the app - it will appear in your menu bar

### Windows

1. Download the latest installer from the [Releases](../../releases) page
2. Run the installer and follow the prompts
3. Timesdump will start automatically and appear in your system tray

## How to Build

### Prerequisites

#### macOS
- macOS 14.0 (Sonoma) or later
- Xcode 15.0 or later
- Swift 6

#### Windows
- Windows 10 (Build 19041) or later
- .NET 10 SDK
- Visual Studio 2022 with WinUI 3 workload (optional)

### Building macOS

```bash
cd macos/Timesdump
xcodebuild build \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -configuration Release
```

### Building Windows

```bash
cd windows
dotnet restore Timesdump.sln
dotnet build Timesdump.sln --configuration Release
```

### Running Tests

#### macOS
```bash
cd macos/Timesdump
xcodebuild test \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS'
```

#### Windows
```bash
cd windows
dotnet test Timesdump.Tests/Timesdump.Tests.csproj
```

## Usage

1. **Copy a timestamp**: Select and copy any Unix timestamp (e.g., `1703462400` or `1703462400000`)
2. **See the result**: A HUD overlay will appear showing the formatted date and time
3. **Copy formatted time**: Click the HUD to copy the formatted time back to your clipboard

### Supported Formats

| Input | Detection | Example Output |
|-------|-----------|----------------|
| 10 digits or less | Seconds | 2023-12-25 00:00:00 |
| More than 10 digits | Milliseconds | 2023-12-25 00:00:00 |

### Filtering

Timesdump uses a year range filter (default: 1990-2050) to automatically ignore:
- Phone numbers
- Verification codes
- Other numeric data that isn't a valid timestamp

## Settings

Access settings through the menu bar (macOS) or system tray (Windows):

### General
- **Launch at Login**: Start Timesdump automatically when you log in
- **HUD Position**: Choose where the overlay appears (Follow Mouse, or fixed corners)
- **Display Duration**: How long the HUD stays visible (1.5s - 10s)
- **Time Format**: Customize the date/time format

### Filter
- **Year Range**: Set the minimum and maximum years for valid timestamps

## Project Structure

```
timesdump/
├── macos/
│   └── Timesdump/
│       ├── Timesdump.xcodeproj
│       ├── Timesdump/
│       │   ├── Models/
│       │   ├── Services/
│       │   ├── Views/
│       │   └── Resources/
│       └── TimesdumpTests/
├── windows/
│   ├── Timesdump.sln
│   ├── Timesdump/
│   │   ├── Models/
│   │   ├── Services/
│   │   ├── Views/
│   │   └── Strings/
│   └── Timesdump.Tests/
├── .github/
│   └── workflows/
│       └── build.yml
├── DESIGN.md
├── README.md
└── LICENSE
```

## Technical Details

### Core Logic (Shared Behavior)

1. **Data Cleaning**: Trim leading/trailing whitespace
2. **Format Validation**: Only pure numeric strings are processed
3. **Unit Detection**: 
   - ≤10 digits → Parse as seconds
   - >10 digits → Parse as milliseconds
4. **Range Filtering**: Validate the resulting year falls within configured range

### macOS Implementation

- **Language**: Swift 6
- **UI Framework**: SwiftUI (Views) + AppKit (Window Management)
- **Window**: `NSPanel` with `.nonactivatingPanel` style mask
- **Clipboard**: `NSPasteboard.general.changeCount` polling (0.5s interval)

### Windows Implementation

- **Language**: C# (.NET 10)
- **UI Framework**: WinUI 3 (Windows App SDK)
- **Window**: Custom presenter with `WS_EX_NOACTIVATE` style
- **Clipboard**: Win32 API polling (0.5s interval)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the need for a simple, non-intrusive timestamp converter
- Built with native platform technologies for the best user experience

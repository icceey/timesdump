# Copilot Instructions for Timesdump

## Project Overview

Timesdump is a cross-platform utility that monitors the clipboard for Unix timestamps and displays them as human-readable dates in a non-intrusive HUD overlay. The project consists of two platform-specific implementations:

- **macOS**: Swift 6 / SwiftUI / AppKit
- **Windows**: C# / .NET 10 / WinUI 3

## Platform-Specific Development

### macOS Development

- **Language**: Swift 6
- **UI Framework**: SwiftUI (Views) + AppKit (Window Management)
- **Minimum OS**: macOS 14.0 (Sonoma)
- **IDE**: Xcode 15.0+
- **Project Location**: `macos/Timesdump/`

#### Build Commands (macOS)
```bash
cd macos/Timesdump
xcodebuild build \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -configuration Release \
  CODE_SIGN_IDENTITY="-"
```

#### Test Commands (macOS)
```bash
cd macos/Timesdump
xcodebuild test \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  CODE_SIGN_IDENTITY="-"
```

### Windows Development

- **Language**: C# (.NET 10)
- **UI Framework**: WinUI 3 (Windows App SDK)
- **Minimum OS**: Windows 10 (Build 19041)
- **Project Location**: `windows/`

#### Build Commands (Windows)
```bash
cd windows
dotnet restore Timesdump.sln
dotnet build Timesdump.sln --configuration Release
```

#### Test Commands (Windows)
```bash
cd windows
dotnet test Timesdump.Tests/Timesdump.Tests.csproj --configuration Release
```

## Code Signing

- **macOS**: Use ad-hoc signing with `CODE_SIGN_IDENTITY="-"` for CI builds (required for Apple Silicon)
- **Windows**: No special signing required for development builds

## Branch Strategy

- **Main Branch**: `master` (not `main`)
- **Development Branch**: `develop`

## CI/CD Workflows

The project uses GitHub Actions for CI/CD:

- **ci.yml**: Lint and test on push to `master`/`develop` and PRs
- **artifacts.yml**: Build and upload artifacts (same triggers + manual dispatch)
- **release.yml**: Triggered on `v*` tags to publish releases
- **build.yml**: Legacy build workflow

## Testing

### macOS Tests
- Located in `macos/Timesdump/TimesdumpTests/`
- Uses XCTest framework

### Windows Tests
- Located in `windows/Timesdump.Tests/`
- Uses xUnit framework
- Requires `using Xunit;` import

## Key Components

### Core Logic (Shared Behavior)
1. **Data Cleaning**: Trim leading/trailing whitespace
2. **Format Validation**: Only pure numeric strings are processed
3. **Unit Detection**: ≤10 digits → seconds, >10 digits → milliseconds
4. **Range Filtering**: Year must fall within configured range (default: 1990-2050)

### Platform-Specific Files

#### macOS
- `Timesdump/Models/` - Data models
- `Timesdump/Services/` - Business logic (TimeParser, ClipboardMonitor)
- `Timesdump/Views/` - SwiftUI views
- `Timesdump/Resources/` - Localization files

#### Windows
- `Timesdump/Models/` - Data models
- `Timesdump/Services/` - Business logic
- `Timesdump/Views/` - WinUI 3 views
- `Timesdump/Strings/` - Localization resources

## Localization

The app supports:
- English (en)
- Simplified Chinese (zh-Hans)

## Common Issues

1. **Xcode Version**: Do not hardcode Xcode versions in workflows; use default `Xcode.app`
2. **Code Signing on macOS**: Always use `CODE_SIGN_IDENTITY="-"` for CI builds
3. **xUnit in .NET**: Ensure `using Xunit;` is present in test files
4. **Resource Paths**: In Xcode project, file paths are relative to their parent group

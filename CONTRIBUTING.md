# Contributing to Timesdump

Thank you for your interest in contributing to Timesdump! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Git](https://git-scm.com/)

#### Platform-specific Requirements

- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Windows**: Visual Studio Build Tools with C++ workload
- **Linux**: Required system libraries:
  ```bash
  sudo apt-get install -y \
    pkg-config build-essential \
    libglib2.0-dev libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev libssl-dev
  ```

### Setting Up the Development Environment

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/timesdump.git
   cd timesdump
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Run in development mode**:
   ```bash
   npm run tauri dev
   ```

## Development Workflow

### Project Structure

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

### Code Style

- **TypeScript/JavaScript**: Follow the existing code style in the project
- **Rust**: Use `rustfmt` for formatting and `clippy` for linting
  ```bash
  cd src-tauri
  cargo fmt --all
  cargo clippy --all-targets -- -D warnings
  ```

### Running Tests

```bash
# Frontend type checking
npx tsc --noEmit

# Rust tests
cd src-tauri
cargo test --lib
```

### Building for Production

```bash
npm run tauri build
```

## Making Contributions

### Reporting Issues

Before creating an issue, please:
1. Search existing issues to avoid duplicates
2. Use a clear and descriptive title
3. Provide detailed information about the problem
4. Include steps to reproduce (if applicable)
5. Mention your operating system and version

### Submitting Pull Requests

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines

3. **Test your changes**:
   - Ensure the app builds without errors
   - Test on your platform
   - Run linting and tests

4. **Commit your changes**:
   - Use clear, descriptive commit messages
   - Reference related issues in commits (e.g., `Fixes #123`)

5. **Push and create a Pull Request**:
   ```bash
   git push origin feature/your-feature-name
   ```
   Then open a PR on GitHub with a clear description of your changes.

### Pull Request Guidelines

- Keep PRs focused and small when possible
- Update documentation if needed
- Add tests for new functionality
- Ensure CI checks pass
- Be responsive to feedback and review comments

## Localization

Timesdump supports multiple languages. To add a new language:

1. Create a new translation file in `src/locales/`
2. Follow the structure of existing translation files
3. Update the language detection logic in `src/lib/i18n.ts`

## Questions?

If you have questions, feel free to:
- Open an issue for discussion
- Check existing documentation and issues

Thank you for contributing to Timesdump! 🎉

# Timesdump

The Silent Timestamp Decoder.

## Project Structure

This is a Rust + Tauri template project for Timesdump.

### Directory Layout

```
timesdump/
├── design.md                    # Product design document
├── index.html                   # Frontend HTML
├── src-tauri/                   # Rust backend
│   ├── Cargo.toml              # Rust dependencies
│   ├── build.rs                # Build script
│   ├── tauri.conf.json         # Tauri configuration
│   ├── src/
│   │   ├── main.rs             # Application entry point
│   │   └── lib.rs              # Library module
│   └── icons/                  # Application icons (placeholder)
└── .github/workflows/
    └── rust.yml                # CI/CD workflow
```

## Development

### Prerequisites

- Rust (latest stable version)
- Node.js (for frontend development)

### Build

```bash
cd src-tauri
cargo build
```

### Test

```bash
cd src-tauri
cargo test
```

### Lint

```bash
cd src-tauri
cargo clippy --all-targets --all-features
```

## CI/CD

The project uses GitHub Actions for continuous integration:
- **Build**: Compiles the Rust code
- **Lint**: Runs Clippy for code quality
- **Test**: Runs unit tests

Workflows are triggered on:
- Pushes to `master` branch
- Pull requests targeting `master` branch

## Error Handling

This project uses `anyhow` for error handling, providing ergonomic error management throughout the codebase.

## License

See LICENSE file for details.

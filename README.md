# YouTube Summarizer — Frontend

This repository contains the frontend for the YouTube Summarizer application. It's built with the Leptos web framework (Rust + WASM) and provides a UI to submit YouTube video URLs and display generated summaries (rendered from Markdown).

**Quick Links**
- Source: `src/`
- Static assets: `assets/`
- Styles: `style/` (Sass)

## Tech stack
- Rust + Leptos
- WASM via `wasm-bindgen`
- Sass for styles
- Playwright for end-to-end tests

## Prerequisites
- Rust toolchain (stable) — install via `rustup`
- `cargo-leptos` (used for dev/packaging tasks)
- Node.js and `npm` (for end-to-end tests)

Install `cargo-leptos` (if not already installed):

```bash
cargo install cargo-leptos --locked
```

## Development

1. Install end-to-end test dependencies (once):

```bash
cd end2end
npm ci
cd ..
```

2. Start the development server (auto-rebuild + reload):

```bash
cargo leptos watch
```

By default the site metadata is configured to serve at `http://127.0.0.1:3000` (see `Cargo.toml`). The compiled site output is written to `target/site`.

## Build for production

Build the WASM bundle and site assets for release:

```bash
cargo leptos build --release --lib
```

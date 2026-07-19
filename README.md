<div align="center">

# Ultralearn

**Build structured, self-directed learning plans grounded in the 9 Ultralearning principles.**

[![CI](https://github.com/suradet-ps/ultralearn/actions/workflows/ci.yml/badge.svg)](https://github.com/suradet-ps/ultralearn/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.97-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.8-dea584?logo=rust&logoColor=white)](https://leptos.dev/)
[![Trunk](https://img.shields.io/badge/Trunk-WASM-ff69b4)](https://trunkrs.dev/)
[![License](https://img.shields.io/badge/license-MIT-4b32c3)](LICENSE)

</div>

---

> Turn any topic — *"Learn Rust"*, *"Play guitar"*, *"Japanese"* — into a guided plan organized around the nine ultralearning principles from [Scott Young](https://www.scotthyoung.com/blog/the-ultralearning-book/).

Ultralearn is a fully client-side web app compiled to WebAssembly with [Leptos](https://leptos.dev/). Enter a topic, and it scaffolds a plan across all 9 principles. Track notes, checklists, flashcards, feedback logs, experiments, and spaced-repetition schedules per principle, and watch your progress compound.

Everything runs in the browser. There is **no backend and no account** — your plans are stored locally in `localStorage` and can be exported or imported as JSON.

---

## Features

| | |
|---|---|
| 🧭 **Plan generator** | Enter a topic and an optional goal to scaffold a plan across all 9 principles. |
| 📚 **9 principles** | Metalearning, Focus, Directness, Drill, Retrieval, Feedback, Retention, Intuition, Experimentation. |
| ✅ **Checklists & notes** | Per-principle prompts, custom checklists, and free-form notes. |
| 🃏 **Flashcards** | Create retrieval practice cards for any principle. |
| 📝 **Feedback log** | Capture outcome / informational / corrective feedback. |
| 🧪 **Experiments** | Track learning hypotheses, methods, and results. |
| 🗓️ **Retention schedules** | Spaced-repetition reminders (1/3/7/14/30-day intervals). |
| 📊 **Progress tracking** | Per-principle and overall completion percentages. |
| 🌗 **Dark mode** | Theme toggle persisted to `localStorage`. |
| 💾 **Export / Import** | Backup or share plans as JSON. |

---

## Tech Stack

- **[Rust](https://www.rust-lang.org/)** (edition 2024, `1.97+`) compiled to `wasm32-unknown-unknown`
- **[Leptos](https://leptos.dev/)** `0.8` — reactive UI (`csr` mode)
- **[leptos_meta](https://docs.rs/leptos_meta)** — document head / title
- **[leptos_router](https://docs.rs/leptos_router)** — client-side routing
- **[Trunk](https://trunkrs.dev/)** — WASM bundler & dev server
- **[gloo-storage](https://docs.rs/gloo-storage)** — `localStorage` persistence
- **[serde](https://serde.rs/)** / **[chrono](https://docs.rs/chrono)** — (de)serialization & dates

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, `1.97+`) with the `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/#install)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### Development

Start the Trunk dev server with hot reload:

```bash
trunk serve
```

The app is served at `http://127.0.0.1:3000` by default.

### Production Build

```bash
trunk build --release
```

Static output is written to `dist/` and can be served by any static host.

### Preview the Build

```bash
trunk serve --release
```

---

## Project Structure

```text
ultralearn/
├── .github/workflows/ci.yml   # CI: fmt, clippy, check, test, trunk build
├── public/                    # Static assets (favicon, styles)
├── src/
│   ├── app.rs                 # Root component (router + shell)
│   ├── lib.rs                 # wasm-bindgen entry point
│   ├── components/            # UI components (layout, icons, activities)
│   ├── composables/           # Reusable logic (timers, etc.)
│   ├── core/                  # Framework-agnostic logic (types, theme, time)
│   ├── stores/                # Reactive plan state (OnceLock singletons)
│   └── views/                 # Route-level pages
├── index.html                 # Trunk entry point
├── Cargo.toml
├── Trunk.toml
└── rust-toolchain.toml
```

---

## Data & Privacy

Ultralearn is fully client-side. All plans, notes, and progress live in your browser's `localStorage` under the key `ultralearn-plans` (and theme preference under `ultralearn-theme`). Clearing your browser data removes your plans — use **Export** in a plan to back up as JSON.

---

## Continuous Integration

GitHub Actions runs on every push and pull request to `main`. The workflow checks formatting (`cargo fmt`), lints (`cargo clippy` for `wasm32`), type-checks (`cargo check`), runs tests (`cargo test`), and produces a production build with Trunk. See [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

---

## Scripts

| Command | Description |
| --- | --- |
| `trunk serve` | Start the development server with hot reload. |
| `trunk build --release` | Build the optimized WASM app to `dist/`. |
| `cargo fmt --all --check` | Verify code formatting. |
| `cargo clippy --target wasm32-unknown-unknown` | Lint the app. |
| `cargo test` | Run unit tests. |

---

## License

Licensed under the [MIT License](LICENSE).
